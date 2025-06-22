# Swoop Platform - v0 UI Scaffolding Prompts

## **v0 Prompt - Foundation Dashboard**

Create a modern document intelligence platform dashboard called "Swoop" with the following specifications:

**Layout & Design:**
- Clean, professional interface with a sidebar navigation
- Main content area with a grid layout for key sections
- Use shadcn/ui components with a dark/light theme toggle
- Responsive design that works on desktop and mobile

**Core Sections:**
1. **Upload Area** - Large drag-and-drop zone with file upload capabilities
2. **Processing Queue** - List showing document processing status with progress bars
3. **Quick Stats** - Cards showing total documents, processing speed, and quality scores
4. **Recent Activity** - Timeline of recent document processing events

**Navigation Sidebar:**
- Dashboard (home icon)
- Documents (folder icon) 
- Chat (message-square icon)
- Settings (settings icon)

**Features:**
- Mock data for demonstration (no real API calls)
- File upload with visual feedback
- Processing status indicators (pending, processing, completed, error)
- Quality score visualization with progress rings
- Activity timeline with timestamps

**Color Scheme:**
- Professional blue/gray palette
- Success green for completed items
- Warning orange for processing
- Error red for failed items

**Components Needed:**
- Drag & drop file upload zone
- Progress bars and status indicators  
- Stat cards with icons and numbers
- Activity timeline component
- Responsive sidebar navigation

Make it look modern, professional, and suitable for enterprise use. Include sample data to show how it would look with real documents being processed.

---

## **v1 Prompt - Enhanced Chat Interface**

Extend the Swoop platform with an advanced AI chat interface:

**New Chat Section:**
- Full-height chat interface with message history
- Support for @ document tagging (e.g., "@report.pdf", "@folder/")
- AI personality selector (Professional, Technical, Casual)
- Message input with autocomplete for document references

**Chat Features:**
1. **Message Types:**
   - User messages with @ document references highlighted
   - AI responses with different personality tones
   - System messages for processing updates
   - Document attachment previews in chat

2. **Document Reference System:**
   - @ symbol triggers autocomplete dropdown
   - Shows available documents and folders
   - Visual tags for referenced documents in messages
   - Click to preview referenced documents

3. **Personality Selector:**
   - Dropdown or toggle for AI personalities
   - Visual indicators showing current personality
   - Different response styles based on selection

**Enhanced Features:**
- Chat history with search functionality
- Export chat conversations
- Document preview modal when clicking references
- Typing indicators and message status
- Copy/share individual messages

**Sample Conversations:**
Include mock conversations showing:
- "@technical_report.pdf What are the key findings?"
- "Analyze @contracts/ for compliance issues"
- Different personality responses to same questions

**UI Improvements:**
- Split view: documents list on left, chat on right
- Collapsible sections for better space management
- Keyboard shortcuts for common actions
- Context menu for messages (copy, delete, export)

---

## **v2 Prompt - Advanced Analytics & Monitoring**

Add comprehensive analytics and monitoring capabilities to Swoop:

**New Analytics Dashboard:**
1. **Processing Metrics:**
   - Real-time processing throughput charts
   - Document type distribution (pie charts)
   - Quality score trends over time
   - Error rate monitoring with alerts

2. **Performance Monitoring:**
   - System health indicators
   - Processing time histograms  
   - Memory and CPU usage graphs
   - Queue depth and processing backlog

3. **Document Intelligence:**
   - Language detection statistics
   - Content classification breakdown
   - PII detection summary (without showing actual PII)
   - Document complexity scoring

**Enhanced Document Management:**
1. **Advanced Filters:**
   - Filter by date range, document type, quality score
   - Search with fuzzy matching
   - Bulk operations (delete, export, reprocess)
   - Sorting by various metrics

2. **Document Details View:**
   - Detailed metadata display
   - Processing history timeline
   - Quality breakdown by section
   - Related documents suggestions

**Monitoring Features:**
- Alert system for processing failures
- Customizable dashboard widgets
- Export analytics data
- Historical trend analysis
- Performance benchmarking

**Visual Components:**
- Interactive charts using recharts or similar
- Heatmaps for processing patterns
- Gauge charts for system health
- Timeline components for processing history
- Data tables with advanced filtering

**Sample Data:**
Include realistic mock data showing:
- 1000+ processed documents
- Various document types (PDF, DOC, TXT, etc.)
- Processing times ranging from seconds to minutes
- Quality scores from 0.7 to 0.98
- Multiple languages detected
- Different error types and frequencies

Make the analytics feel comprehensive and enterprise-ready, with the ability to drill down into specific metrics and time periods.

---

## **Implementation Notes for v0 Development:**

**Progressive Enhancement:**
- Start with v0 (foundation) - focus on core layout and upload
- Move to v1 (chat) - add AI interaction capabilities  
- Finish with v2 (analytics) - add comprehensive monitoring

**Mock Data Strategy:**
- Use realistic but fake document names and metadata
- Include various processing states and quality scores
- Show different document types and languages
- Demonstrate error handling and edge cases

**No API Dependencies:**
- All data should be mocked/hardcoded
- Use setTimeout to simulate processing delays
- Include loading states and transitions
- Focus on UI/UX rather than backend integration

**Responsive Design:**
- Mobile-first approach
- Collapsible sidebar for smaller screens
- Touch-friendly interface elements
- Proper spacing and typography scaling

**Accessibility:**
- Proper ARIA labels and roles
- Keyboard navigation support
- Color contrast compliance
- Screen reader friendly

Each prompt builds upon the previous one, creating a comprehensive document intelligence platform that demonstrates all the key features without requiring backend APIs. 