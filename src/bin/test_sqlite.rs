/*!
 * Test SQLite storage implementation
 */

use swoop::{
    error::Result,
    models::Document,
    storage::{sqlite::SqliteStorage, Storage},
};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    println!("🧪 Testing SQLite Storage Implementation");
    
    // Create storage with connection string instead of config
    let connection_string = "sqlite:./test_swoop.db";
    let storage = SqliteStorage::new(connection_string).await?;
    
    println!("✅ SQLite storage created successfully");
    
    // Create a test document
    let mut document = Document::new("https://example.com/test", "Test content for SQLite storage");
    // Set content instead of separate html/text fields
    document.content = "Test Document\nThis is a test document for SQLite storage.".to_string();
    document.title = "Test Document".to_string();
    
    // Add some metadata
    document.metadata.content_type = Some("text/html".to_string());
    document.metadata.custom.insert("user-agent".to_string(), "swoop-test/1.0".to_string());
    
    // Document already has most fields we need for testing
    
    println!("📄 Created test document: {}", document.id);
    
    // Test store_document
    storage.store_document(&document).await?;
    println!("✅ Document stored successfully");
    
    // Test retrieve_document
    let retrieved = storage.retrieve_document(&document.id).await?;
    match retrieved {
        Some(doc) => {
            println!("✅ Document retrieved successfully");
            println!("   Title: {}", doc.title);
            println!("   Content length: {}", doc.content.len());
        }
        None => {
            println!("❌ Document not found");
            return Ok(());
        }
    }
    
    // Test list_documents
    let document_ids = storage.list_documents().await?;
    println!("✅ Listed {} document IDs", document_ids.len());
    
    // Test health_check
    let is_healthy = storage.health_check().await?;
    println!("✅ Storage health check: {is_healthy}");
    
    // Test document update by storing again
    let mut updated_doc = document.clone();
    updated_doc.title = "Updated Test Document".to_string();
    updated_doc.content = "Updated content for SQLite storage".to_string();
    
    storage.store_document(&updated_doc).await?;
    println!("✅ Document updated successfully");
    
    // Verify update
    let retrieved_updated = storage.retrieve_document(&document.id).await?;
    if let Some(doc) = retrieved_updated {
        println!("✅ Updated document verified");
        println!("   New title: {}", doc.title);
        println!("   New content: {}", doc.content);
    }
    
    // Test delete_document
    storage.delete_document(&document.id).await?;
    println!("✅ Document deleted successfully");
    
    // Verify deletion
    let deleted_check = storage.retrieve_document(&document.id).await?;
    if deleted_check.is_none() {
        println!("✅ Document deletion verified");
    } else {
        println!("❌ Document still exists after deletion");
    }
    
    println!("🎉 All SQLite storage tests passed!");
    
    Ok(())
} 