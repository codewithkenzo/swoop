# Glossary

## A

**AI (Artificial Intelligence)**  
Computer systems designed to perform tasks that typically require human intelligence, such as understanding text, recognizing patterns, and making decisions.

**API (Application Programming Interface)**  
A set of protocols and tools for building software applications, defining how components should interact.

**API Key**  
A unique identifier used to authenticate requests to the Swoop API, ensuring secure access to your documents and data.

**Async (Asynchronous)**  
A programming pattern that allows operations to run concurrently without blocking other processes, essential for handling multiple document processing tasks simultaneously.

**Audit Log**  
A detailed record of all system activities, including user actions, document access, and system changes, used for security and compliance monitoring.

## B

**BM25 (Best Matching 25)**  
A ranking function used in full-text search that determines how relevant a document is to a search query based on term frequency and document length.

**Backend**  
The server-side component of Swoop, built with Rust, that handles document processing, AI analysis, and data storage.

**Batch Processing**  
The ability to process multiple documents simultaneously, improving efficiency for large document collections.

## C

**Categorization**  
The automatic classification of documents into predefined categories (technical, business, legal, academic) using AI analysis.

**Chat Interface**  
Interactive feature that allows users to ask questions about uploaded documents and receive AI-generated responses based on document content.

**Chunking**  
The process of breaking large documents into smaller, manageable pieces for more efficient processing and analysis.

**CORS (Cross-Origin Resource Sharing)**  
A security feature that controls which web domains can access the Swoop API from a browser.

**Crawler**  
A component that systematically browses websites to extract content, following links and respecting robots.txt files.

**cURL**  
A command-line tool for making HTTP requests, commonly used for testing APIs and automation.

## D

**Document Intelligence**  
The application of AI and machine learning to automatically extract, analyze, and understand information from documents.

**Document Metadata**  
Information about a document such as title, creation date, file size, author, and processing status.

**Deployment**  
The process of installing and configuring Swoop on servers or cloud platforms for production use.

## E

**Embedding**  
A mathematical representation of text as a vector of numbers, enabling semantic similarity calculations and advanced search capabilities.

**Entity Recognition (NER)**  
The process of identifying and classifying named entities in text, such as people, organizations, locations, and dates.

**Extraction**  
The process of pulling text content from various document formats (PDF, HTML, etc.) for further processing.

## F

**Frontend**  
The user-facing component of Swoop, built with React and TypeScript, providing web interface for document management.

**Full-text Search**  
A search technique that examines all words in documents to find matches, as opposed to searching only titles or metadata.

## G

**Geocoding**  
The process of converting location names mentioned in documents into geographic coordinates.

**GPU (Graphics Processing Unit)**  
Specialized hardware that can accelerate AI computations, though Swoop primarily uses CPU-based processing.

## H

**Health Check**  
An endpoint or mechanism that verifies Swoop services are running correctly and responding to requests.

**Hybrid Search**  
Swoop's search approach that combines keyword-based (BM25) and semantic (vector) search methods for optimal results.

**HTTP (Hypertext Transfer Protocol)**  
The protocol used for communication between web browsers/clients and the Swoop API server.

## I

**Indexing**  
The process of organizing and storing document content and metadata in a way that enables fast search and retrieval.

**Integration**  
The process of connecting Swoop with other software systems, applications, or services.

**ISO (International Organization for Standardization)**  
Develops international standards, including document formats and security protocols that Swoop supports.

## J

**JSON (JavaScript Object Notation)**  
A lightweight data format used for API communication and configuration files in Swoop.

**JWT (JSON Web Token)**  
A security token format used for authentication and authorization in web applications.

## K

**Kubernetes**  
A container orchestration platform that can be used to deploy and manage Swoop in production environments.

**Key-Value Store**  
A type of database (like Redis) used by Swoop for caching and session management.

## L

**Latency**  
The time delay between making a request and receiving a response, a key performance metric for Swoop.

**LLM (Large Language Model)**  
AI models like GPT-4, Claude, or Llama that can understand and generate human-like text, used by Swoop for document analysis.

**Load Balancing**  
Distributing incoming requests across multiple Swoop server instances to improve performance and reliability.

## M

**Metadata**  
Data that describes other data; in Swoop's context, information about documents like size, type, creation date, and analysis results.

**MDX**  
A format that combines Markdown with React components, used for Swoop's documentation system.

**Microservices**  
An architectural approach where applications are built as a collection of small, independent services.

**MIME Type**  
A standard way of describing the format of a file (e.g., "application/pdf" for PDF files).

## N

**NLP (Natural Language Processing)**  
A branch of AI that helps computers understand, interpret, and manipulate human language.

**Node.js**  
A JavaScript runtime used for building Swoop's frontend and documentation site.

**Normalization**  
The process of converting text to a standard format for consistent processing and comparison.

## O

**OCR (Optical Character Recognition)**  
Technology that converts images of text into machine-readable text format.

**OpenAPI**  
A specification for describing REST APIs, used to document Swoop's API endpoints and data structures.

**OpenRouter**  
A service that provides access to multiple AI models through a single API, used by Swoop for document analysis.

## P

**PDF (Portable Document Format)**  
A file format that preserves document formatting across different platforms, commonly processed by Swoop.

**Pagination**  
The practice of dividing large result sets into smaller, manageable pages for better performance and user experience.

**PostgreSQL**  
An open-source relational database system that Swoop can use for storing document metadata and user information.

**Processing Pipeline**  
The sequence of steps Swoop follows to analyze a document: upload → extraction → analysis → storage → indexing.

## Q

**Quality Score**  
A numerical rating (0-100) assigned by Swoop's AI to indicate document readability, structure, and overall quality.

**Qdrant**  
A vector database used by Swoop to store and search document embeddings for semantic similarity.

**Query**  
A request for information, typically a search term or phrase used to find relevant documents.

## R

**RAG (Retrieval-Augmented Generation)**  
An AI technique that combines document retrieval with text generation, used in Swoop's chat feature.

**Rate Limiting**  
A technique to control the number of API requests a user can make within a specific time period.

**React**  
A JavaScript library for building user interfaces, used for Swoop's frontend application.

**Redis**  
An in-memory data store used by Swoop for caching and session management.

**REST (Representational State Transfer)**  
An architectural style for web APIs that Swoop follows for its HTTP endpoints.

**Robots.txt**  
A file that websites use to communicate with web crawlers about which pages should not be accessed.

**Rust**  
A systems programming language used to build Swoop's high-performance backend.

## S

**Sanitization**  
The process of cleaning and validating input data to prevent security vulnerabilities.

**Semantic Search**  
A search technique that understands the meaning and context of queries, not just exact keyword matches.

**Sentiment Analysis**  
The process of determining the emotional tone or attitude expressed in text (positive, negative, neutral).

**Server-Sent Events (SSE)**  
A web standard that allows servers to push real-time updates to web browsers, used for live progress tracking.

**Similarity Score**  
A numerical value indicating how closely related two documents are based on their content and context.

**SQLite**  
A lightweight, file-based database system used by Swoop for local development and small deployments.

**Streaming**  
The process of sending data in small chunks as it becomes available, rather than waiting for complete processing.

## T

**TLS (Transport Layer Security)**  
A cryptographic protocol that provides secure communication over networks, used to protect API communications.

**Tokenization**  
The process of breaking text into individual words or meaningful units for analysis.

**TTS (Text-to-Speech)**  
Technology that converts written text into spoken audio, integrated with ElevenLabs in Swoop.

**TypeScript**  
A programming language that adds static type checking to JavaScript, used in Swoop's frontend.

## U

**UI (User Interface)**  
The visual elements and controls that users interact with in Swoop's web application.

**URL (Uniform Resource Locator)**  
A web address that identifies a specific resource, used in web crawling and API endpoints.

**UUID (Universally Unique Identifier)**  
A 128-bit identifier used to uniquely identify documents and other objects in Swoop.

**UX (User Experience)**  
The overall experience and satisfaction a user has when interacting with Swoop's interface.

## V

**Vector**  
A mathematical representation of text as an array of numbers, enabling semantic similarity calculations.

**Vector Database**  
A specialized database designed to store and query high-dimensional vectors efficiently.

**Vectorization**  
The process of converting text into numerical vectors that capture semantic meaning.

**Versioning**  
The practice of tracking changes to software over time, following semantic versioning (MAJOR.MINOR.PATCH).

## W

**Webhook**  
A method for applications to receive real-time notifications when events occur, supported by Swoop for integrations.

**Web Crawling**  
The automated process of systematically browsing websites to discover and extract content.

**Word Count**  
The total number of words in a document, calculated during Swoop's content analysis phase.

## X

**XML (eXtensible Markup Language)**  
A markup language that defines document structure, sometimes processed by Swoop's HTML parser.

**XSS (Cross-Site Scripting)**  
A security vulnerability that Swoop protects against through input sanitization and output encoding.

## Y

**YAML (YAML Ain't Markup Language)**  
A human-readable data serialization format used for configuration files and documentation.

## Z

**Zero Downtime Deployment**  
A deployment strategy that updates software without interrupting service availability.

---

## Document Processing Terms

**Analysis Pipeline**  
The complete workflow from document upload through AI analysis to final storage and indexing.

**Content Extraction**  
The process of pulling readable text and metadata from various document formats.

**Document Fingerprinting**  
Creating a unique identifier for a document based on its content to detect duplicates.

**Format Detection**  
Automatically identifying the type of uploaded file (PDF, HTML, etc.) for appropriate processing.

**Quality Assessment**  
AI-based evaluation of document structure, readability, and information value.

## AI and Machine Learning Terms

**Confidence Score**  
A percentage indicating how certain the AI is about its analysis or categorization results.

**Fine-tuning**  
The process of adapting a pre-trained AI model for specific tasks or domains.

**Hallucination**  
When an AI model generates information that isn't present in the source document.

**Model Selection**  
Choosing the most appropriate AI model for a specific task based on requirements and performance.

**Prompt Engineering**  
The practice of crafting effective instructions for AI models to produce desired outputs.

## Search and Retrieval Terms

**Faceted Search**  
Search functionality that allows filtering results by multiple criteria (category, date, quality, etc.).

**Relevance Scoring**  
The process of ranking search results based on how well they match the user's query.

**Search Index**  
A data structure that enables fast full-text search across document collections.

**Stop Words**  
Common words (like "the", "and", "is") that are often ignored during search processing.

## Security and Compliance Terms

**Authentication**  
The process of verifying the identity of users or systems accessing Swoop.

**Authorization**  
Determining what actions an authenticated user is permitted to perform.

**Data Residency**  
The requirement that data must be stored in a specific geographic location for legal compliance.

**Encryption at Rest**  
Protecting stored data by encrypting it when saved to disk or database.

**Encryption in Transit**  
Protecting data by encrypting it while being transmitted over networks.

**GDPR (General Data Protection Regulation)**  
European privacy law that affects how personal data is collected, processed, and stored.

**SOC 2**  
Security framework that defines criteria for managing customer data based on security, availability, and confidentiality.

---

This glossary is regularly updated as Swoop evolves. For the most current definitions or to suggest additions, please visit our [documentation site](https://docs.swoop.dev) or contribute via [GitHub](https://github.com/your-org/swoop).