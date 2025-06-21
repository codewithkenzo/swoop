# Swoop Frontend - React TypeScript Application

This is the modern React TypeScript frontend for the Swoop AI-Powered Document Intelligence Platform. It provides a beautiful, intuitive interface for document management, AI analysis, and web crawling capabilities.

## 🚀 Technology Stack

Following [2025 React best practices][[memory:2911069043609634502]], this frontend uses:

- **React 18+** with TypeScript for type safety
- **Vite** for fast development and building
- **Tailwind CSS** for utility-first styling
- **shadcn/ui** components for accessible, customizable UI
- **React Router** for client-side routing
- **TanStack Query** for server state management
- **React Dropzone** for drag-and-drop file uploads
- **Lucide React** for beautiful icons
- **Zustand** for client-side state management
- **Framer Motion** for smooth animations

## 📁 Project Structure

```
frontend/
├── src/
│   ├── components/
│   │   ├── ui/                 # shadcn/ui components
│   │   ├── layout/             # Layout components
│   │   └── upload/             # Upload-specific components
│   ├── pages/                  # Page components
│   ├── lib/                    # Utilities and API client
│   ├── types/                  # TypeScript type definitions
│   └── hooks/                  # Custom React hooks
├── public/                     # Static assets
└── dist/                       # Build output
```

## 🎨 Features Implemented

### ✅ Core Architecture
- **Modern Layout**: Responsive sidebar navigation with mobile support
- **Type Safety**: Comprehensive TypeScript definitions for all API interactions
- **State Management**: TanStack Query for server state, Zustand for client state
- **Error Handling**: Graceful error boundaries and user feedback

### ✅ Dashboard
- **Overview Statistics**: Document counts, storage usage, AI processing stats
- **Activity Feed**: Real-time updates on document processing
- **Category Analytics**: Visual breakdown of document types
- **Quick Actions**: Fast access to common tasks

### ✅ File Upload System
- **Drag & Drop**: Beautiful, intuitive file upload interface
- **Progress Tracking**: Real-time upload progress with status indicators
- **Format Support**: PDF, Word, Text, HTML, Markdown files
- **AI Configuration**: Toggle analysis features before upload
- **Batch Processing**: Multiple file uploads with individual progress

### ✅ AI Integration Ready
- **Analysis Configuration**: Granular control over AI features
- **Real-time Feedback**: Progress indicators for AI processing
- **Results Display**: Structured presentation of analysis results
- **Confidence Scores**: Visual indicators of AI certainty

## 🛠️ Installation & Setup

1. **Install Dependencies**
   ```bash
   cd frontend
   npm install
   ```

2. **Development Server**
   ```bash
   npm run dev
   ```
   The app will be available at `http://localhost:3000`

3. **Build for Production**
   ```bash
   npm run build
   ```

4. **Preview Production Build**
   ```bash
   npm run preview
   ```

## 🔧 Configuration

### API Integration
The frontend is configured to proxy API requests to the Rust backend:
- Development: `http://localhost:8080/api`
- Production: Configurable via environment variables

### Environment Variables
Create a `.env` file in the frontend directory:
```env
VITE_API_BASE_URL=http://localhost:8080
VITE_APP_NAME=Swoop
VITE_ENABLE_DEVTOOLS=true
```

## 🎯 Key Components

### Layout System
- **Responsive Design**: Mobile-first approach with collapsible sidebar
- **Navigation**: Intuitive routing with active state indicators
- **Theme Support**: Built-in dark/light mode toggle capability

### Upload System
- **FileUpload Component**: Reusable drag-and-drop interface
- **Progress Tracking**: Real-time upload and processing status
- **Error Handling**: User-friendly error messages and retry options

### API Client
- **Type-Safe**: Full TypeScript integration with backend types
- **Error Handling**: Consistent error handling across all requests
- **Caching**: Intelligent caching with TanStack Query

## 🔮 Upcoming Features

### 📋 Document Management
- Document library with advanced filtering
- Bulk operations and batch processing
- Document preview and annotation
- Version history and collaboration

### 🔍 AI-Powered Search
- Semantic search across document collection
- Advanced filters by category, entities, tags
- Search result highlighting and relevance scoring
- Saved searches and search history

### 🕷️ Web Crawling Interface
- Visual URL input with validation
- Crawl job management and monitoring
- Real-time progress tracking
- Crawl result analysis and filtering

### ⚙️ Advanced Settings
- AI model configuration
- Processing pipeline customization
- User preferences and profiles
- System monitoring and health checks

## 🧪 Testing Strategy

- **Unit Tests**: Component testing with React Testing Library
- **Integration Tests**: API integration and user flow testing
- **E2E Tests**: Full application testing with Playwright
- **Type Checking**: Comprehensive TypeScript coverage

## 📱 Mobile Responsiveness

The interface is fully responsive with:
- Collapsible sidebar for mobile devices
- Touch-friendly interaction elements
- Optimized layouts for tablet and phone screens
- Progressive Web App (PWA) capabilities

## 🎨 Design System

Following modern design principles:
- **Accessibility**: WCAG 2.1 AA compliance
- **Consistency**: Unified component library
- **Performance**: Optimized for fast loading
- **Scalability**: Modular architecture for easy extension

## 🚀 Deployment

The frontend can be deployed to:
- **Vercel**: Optimized for React applications
- **Netlify**: Static site hosting with edge functions
- **AWS S3 + CloudFront**: Enterprise-grade hosting
- **Docker**: Containerized deployment

## 📖 Development Guidelines

1. **Component Structure**: Keep components under 200-300 lines
2. **Type Safety**: Always define explicit TypeScript interfaces
3. **Accessibility**: Use semantic HTML and ARIA attributes
4. **Performance**: Lazy load routes and optimize bundle size
5. **Testing**: Write tests for all critical user flows

This frontend provides a solid foundation for the Swoop platform, bridging the gap between the powerful Rust backend and an intuitive user experience. The modular architecture supports easy extension and customization as the platform evolves. 