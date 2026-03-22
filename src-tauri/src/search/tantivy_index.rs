use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{doc, Index, IndexReader, IndexWriter, ReloadPolicy, TantivyDocument};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit {
    pub path: String,
    pub score: f32,
    pub snippet: String,
}

/// Wrapper around a Tantivy index tailored for document search.
pub struct SearchIndex {
    index: Index,
    reader: IndexReader,
    writer: IndexWriter,
    schema: Schema,
    // Field handles kept for convenience.
    f_path: Field,
    f_filename: Field,
    f_content: Field,
    f_extension: Field,
    f_modified_at: Field,
}

impl SearchIndex {
    /// Creates or opens a Tantivy index at the given directory.
    pub fn new(path: &Path) -> Result<Self> {
        std::fs::create_dir_all(path)
            .with_context(|| format!("Failed to create index directory: {}", path.display()))?;

        let mut schema_builder = Schema::builder();

        let f_path = schema_builder.add_text_field("path", STRING | STORED);
        let f_filename = schema_builder.add_text_field("filename", TEXT | STORED);
        let f_content = schema_builder.add_text_field("content", TEXT);
        let f_extension = schema_builder.add_text_field("extension", STRING | STORED);
        let f_modified_at = schema_builder.add_text_field("modified_at", STORED);

        let schema = schema_builder.build();

        let index = if Index::exists(path)? {
            Index::open_in_dir(path)
                .with_context(|| "Failed to open existing Tantivy index")?
        } else {
            Index::create_in_dir(path, schema.clone())
                .with_context(|| "Failed to create Tantivy index")?
        };

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .with_context(|| "Failed to create index reader")?;

        // 50 MB writer heap.
        let writer = index
            .writer(50_000_000)
            .with_context(|| "Failed to create index writer")?;

        Ok(Self {
            index,
            reader,
            writer,
            schema,
            f_path,
            f_filename,
            f_content,
            f_extension,
            f_modified_at,
        })
    }

    /// Adds a single document to the index (not yet committed).
    pub fn add_document(
        &mut self,
        path: &str,
        filename: &str,
        content: &str,
        extension: &str,
        modified_at: &str,
    ) -> Result<()> {
        // Remove any previous document with the same path first.
        let path_term = tantivy::Term::from_field_text(self.f_path, path);
        self.writer.delete_term(path_term);

        self.writer.add_document(doc!(
            self.f_path => path,
            self.f_filename => filename,
            self.f_content => content,
            self.f_extension => extension,
            self.f_modified_at => modified_at,
        ))?;

        Ok(())
    }

    /// Searches the index and returns up to `limit` hits.
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchHit>> {
        let searcher = self.reader.searcher();

        let query_parser =
            QueryParser::for_index(&self.index, vec![self.f_filename, self.f_content]);

        let parsed = query_parser
            .parse_query(query)
            .with_context(|| format!("Failed to parse query: {}", query))?;

        let top_docs = searcher.search(&parsed, &TopDocs::with_limit(limit))?;

        let snippet_generator =
            tantivy::SnippetGenerator::create(&searcher, &parsed, self.f_content)?;

        let mut hits = Vec::with_capacity(top_docs.len());
        for (score, doc_addr) in top_docs {
            let retrieved: TantivyDocument = searcher.doc(doc_addr)?;

            let path = retrieved
                .get_first(self.f_path)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let snippet = snippet_generator.snippet_from_doc(&retrieved);
            let snippet_text = snippet.to_html();

            hits.push(SearchHit {
                path,
                score,
                snippet: snippet_text,
            });
        }

        Ok(hits)
    }

    /// Deletes all documents matching the given path.
    pub fn delete_by_path(&mut self, path: &str) -> Result<()> {
        let term = tantivy::Term::from_field_text(self.f_path, path);
        self.writer.delete_term(term);
        Ok(())
    }

    /// Commits pending changes to the index.
    pub fn commit(&mut self) -> Result<()> {
        self.writer.commit()?;
        self.reader.reload()?;
        Ok(())
    }
}
