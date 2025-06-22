"use client"

import { useState, useRef, useEffect } from "react"
import { Send, Search, Download, Copy, Trash2, FileText, Folder, Bot, User, Settings2, Paperclip } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/components/ui/dropdown-menu"
import { ScrollArea } from "@/components/ui/scroll-area"
import { Separator } from "@/components/ui/separator"
import { cn } from "@/lib/utils"

// Mock data
const mockDocuments = [
  { id: 1, name: "technical_report.pdf", type: "file", path: "/documents/technical_report.pdf" },
  { id: 2, name: "financial_summary.xlsx", type: "file", path: "/documents/financial_summary.xlsx" },
  { id: 3, name: "contracts", type: "folder", path: "/documents/contracts/" },
  { id: 4, name: "meeting_notes.docx", type: "file", path: "/documents/meeting_notes.docx" },
  { id: 5, name: "legal", type: "folder", path: "/documents/legal/" },
  { id: 6, name: "quarterly_review.pptx", type: "file", path: "/documents/quarterly_review.pptx" },
]

const mockMessages = [
  {
    id: 1,
    type: "user",
    content: "Hello! Can you help me analyze @technical_report.pdf?",
    timestamp: new Date(Date.now() - 3600000),
    references: [{ name: "technical_report.pdf", type: "file" }],
  },
  {
    id: 2,
    type: "ai",
    content:
      "I'd be happy to help you analyze the technical report! I can see you've referenced technical_report.pdf. Based on the document, here are the key findings:\n\n• Performance metrics show a 15% improvement\n• Technical debt has been reduced by 23%\n• Three critical issues were identified and resolved\n\nWould you like me to dive deeper into any specific section?",
    timestamp: new Date(Date.now() - 3590000),
    personality: "professional",
  },
  {
    id: 3,
    type: "user",
    content: "What about compliance issues in @contracts/?",
    timestamp: new Date(Date.now() - 3000000),
    references: [{ name: "contracts", type: "folder" }],
  },
  {
    id: 4,
    type: "ai",
    content:
      "Great question! I've analyzed the contracts folder and found several important compliance considerations:\n\n🔍 **Compliance Status:**\n• 12 contracts reviewed\n• 2 require immediate attention\n• 8 are fully compliant\n• 2 pending legal review\n\n⚠️ **Key Issues:**\n• Missing data protection clauses in vendor agreements\n• Outdated termination procedures in 3 contracts\n\nShall I generate a detailed compliance report?",
    timestamp: new Date(Date.now() - 2990000),
    personality: "professional",
  },
  {
    id: 5,
    type: "system",
    content: "Document processing completed for quarterly_review.pptx",
    timestamp: new Date(Date.now() - 1800000),
  },
]

const personalities = [
  { id: "professional", name: "Professional", description: "Formal and business-focused responses" },
  { id: "technical", name: "Technical", description: "Detailed technical analysis and explanations" },
  { id: "casual", name: "Casual", description: "Friendly and conversational tone" },
]

interface DocumentReference {
  name: string
  type: "file" | "folder"
}

interface Message {
  id: number
  type: "user" | "ai" | "system"
  content: string
  timestamp: Date
  references?: DocumentReference[]
  personality?: string
}

function DocumentPreviewModal({ document, isOpen, onClose }: { document: any; isOpen: boolean; onClose: () => void }) {
  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent className="max-w-4xl max-h-[80vh]">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            {document?.type === "folder" ? <Folder className="h-5 w-5" /> : <FileText className="h-5 w-5" />}
            {document?.name}
          </DialogTitle>
        </DialogHeader>
        <div className="p-4 bg-muted rounded-lg">
          <p className="text-sm text-muted-foreground mb-4">Document Preview</p>
          {document?.type === "folder" ? (
            <div className="space-y-2">
              <p className="font-medium">Folder Contents:</p>
              <ul className="list-disc list-inside space-y-1 text-sm">
                <li>contract_template.docx</li>
                <li>vendor_agreement_2024.pdf</li>
                <li>service_level_agreement.pdf</li>
                <li>partnership_contract.docx</li>
              </ul>
            </div>
          ) : (
            <div className="space-y-4">
              <div className="bg-white dark:bg-gray-800 p-4 rounded border">
                <h3 className="font-semibold mb-2">Executive Summary</h3>
                <p className="text-sm">
                  This technical report outlines the quarterly performance metrics and system improvements implemented
                  during Q3 2024. Key achievements include performance optimization, security enhancements, and
                  technical debt reduction.
                </p>
              </div>
              <div className="bg-white dark:bg-gray-800 p-4 rounded border">
                <h3 className="font-semibold mb-2">Key Metrics</h3>
                <ul className="text-sm space-y-1">
                  <li>• System uptime: 99.9%</li>
                  <li>• Response time improvement: 15%</li>
                  <li>• Bug resolution rate: 95%</li>
                </ul>
              </div>
            </div>
          )}
        </div>
      </DialogContent>
    </Dialog>
  )
}

function MessageComponent({ message, onDocumentClick }: { message: Message; onDocumentClick: (doc: any) => void }) {
  const formatTime = (date: Date) => {
    return date.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })
  }

  const renderMessageContent = (content: string, references?: DocumentReference[]) => {
    if (!references || references.length === 0) return content

    let processedContent = content
    references.forEach((ref) => {
      const refPattern = new RegExp(`@${ref.name}`, "g")
      processedContent = processedContent.replace(
        refPattern,
        `<span class="document-reference" data-doc="${ref.name}" data-type="${ref.type}">@${ref.name}</span>`,
      )
    })

    return (
      <div
        dangerouslySetInnerHTML={{ __html: processedContent }}
        onClick={(e) => {
          const target = e.target as HTMLElement
          if (target.classList.contains("document-reference")) {
            const docName = target.getAttribute("data-doc")
            const docType = target.getAttribute("data-type")
            const doc = mockDocuments.find((d) => d.name === docName)
            if (doc) onDocumentClick(doc)
          }
        }}
      />
    )
  }

  if (message.type === "system") {
    return (
      <div className="flex justify-center my-4">
        <div className="bg-muted px-3 py-1 rounded-full text-xs text-muted-foreground">{message.content}</div>
      </div>
    )
  }

  return (
    <div className={cn("flex gap-3 mb-6", message.type === "user" ? "justify-end" : "justify-start")}>
      {message.type === "ai" && (
        <div className="flex-shrink-0">
          <div className="w-8 h-8 rounded-full bg-primary flex items-center justify-center">
            <Bot className="h-4 w-4 text-primary-foreground" />
          </div>
        </div>
      )}
      <div className={cn("max-w-[80%] space-y-2", message.type === "user" ? "order-first" : "")}>
        <div
          className={cn(
            "rounded-lg px-4 py-2 break-words",
            message.type === "user" ? "bg-primary text-primary-foreground ml-auto" : "bg-muted",
          )}
        >
          <div className="whitespace-pre-wrap break-words">
            {renderMessageContent(message.content, message.references)}
          </div>
          {message.references && message.references.length > 0 && (
            <div className="flex flex-wrap gap-1 mt-2">
              {message.references.map((ref, index) => (
                <Badge
                  key={index}
                  variant="secondary"
                  className="text-xs cursor-pointer hover:bg-secondary/80"
                  onClick={() => {
                    const doc = mockDocuments.find((d) => d.name === ref.name)
                    if (doc) onDocumentClick(doc)
                  }}
                >
                  {ref.type === "folder" ? <Folder className="h-3 w-3 mr-1" /> : <FileText className="h-3 w-3 mr-1" />}
                  {ref.name}
                </Badge>
              ))}
            </div>
          )}
        </div>
        <div className="flex items-center gap-2 text-xs text-muted-foreground">
          <span>{formatTime(message.timestamp)}</span>
          {message.personality && (
            <Badge variant="outline" className="text-xs">
              {personalities.find((p) => p.id === message.personality)?.name}
            </Badge>
          )}
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="ghost" size="sm" className="h-6 w-6 p-0">
                <Settings2 className="h-3 w-3" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent>
              <DropdownMenuItem>
                <Copy className="h-4 w-4 mr-2" />
                Copy
              </DropdownMenuItem>
              <DropdownMenuItem>
                <Download className="h-4 w-4 mr-2" />
                Export
              </DropdownMenuItem>
              {message.type === "user" && (
                <DropdownMenuItem className="text-destructive">
                  <Trash2 className="h-4 w-4 mr-2" />
                  Delete
                </DropdownMenuItem>
              )}
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </div>
      {message.type === "user" && (
        <div className="flex-shrink-0">
          <div className="w-8 h-8 rounded-full bg-muted flex items-center justify-center">
            <User className="h-4 w-4 text-muted-foreground" />
          </div>
        </div>
      )}
    </div>
  )
}

function DocumentAutocomplete({
  isOpen,
  onSelect,
  onClose,
  searchTerm,
}: {
  isOpen: boolean
  onSelect: (doc: any) => void
  onClose: () => void
  searchTerm: string
}) {
  const filteredDocs = mockDocuments.filter((doc) => doc.name.toLowerCase().includes(searchTerm.toLowerCase()))

  if (!isOpen || filteredDocs.length === 0) return null

  return (
    <Card className="absolute bottom-full left-0 right-0 mb-2 z-50">
      <CardContent className="p-2">
        <div className="space-y-1">
          {filteredDocs.slice(0, 5).map((doc) => (
            <div
              key={doc.id}
              className="flex items-center gap-2 p-2 rounded hover:bg-muted cursor-pointer"
              onClick={() => onSelect(doc)}
            >
              {doc.type === "folder" ? <Folder className="h-4 w-4" /> : <FileText className="h-4 w-4" />}
              <span className="text-sm">{doc.name}</span>
              <Badge variant="outline" className="text-xs ml-auto">
                {doc.type}
              </Badge>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  )
}

export default function ChatPage() {
  const [messages, setMessages] = useState<Message[]>(mockMessages)
  const [inputValue, setInputValue] = useState("")
  const [selectedPersonality, setSelectedPersonality] = useState("professional")
  const [isTyping, setIsTyping] = useState(false)
  const [showAutocomplete, setShowAutocomplete] = useState(false)
  const [autocompleteSearch, setAutocompleteSearch] = useState("")
  const [selectedDocument, setSelectedDocument] = useState<any>(null)
  const [searchQuery, setSearchQuery] = useState("")

  const messagesEndRef = useRef<HTMLDivElement>(null)
  const inputRef = useRef<HTMLInputElement>(null)

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" })
  }

  useEffect(() => {
    scrollToBottom()
  }, [messages])

  const handleInputChange = (value: string) => {
    setInputValue(value)

    // Check for @ symbol to trigger autocomplete
    const lastAtIndex = value.lastIndexOf("@")
    if (lastAtIndex !== -1 && lastAtIndex === value.length - 1) {
      setShowAutocomplete(true)
      setAutocompleteSearch("")
    } else if (lastAtIndex !== -1) {
      const searchTerm = value.substring(lastAtIndex + 1)
      if (searchTerm && !searchTerm.includes(" ")) {
        setShowAutocomplete(true)
        setAutocompleteSearch(searchTerm)
      } else {
        setShowAutocomplete(false)
      }
    } else {
      setShowAutocomplete(false)
    }
  }

  const handleDocumentSelect = (doc: any) => {
    const lastAtIndex = inputValue.lastIndexOf("@")
    const newValue = inputValue.substring(0, lastAtIndex) + `@${doc.name} `
    setInputValue(newValue)
    setShowAutocomplete(false)
    inputRef.current?.focus()
  }

  const extractReferences = (content: string): DocumentReference[] => {
    const references: DocumentReference[] = []
    const atMatches = content.match(/@[\w.-]+/g)

    if (atMatches) {
      atMatches.forEach((match) => {
        const docName = match.substring(1)
        const doc = mockDocuments.find((d) => d.name === docName)
        if (doc) {
          references.push({ name: doc.name, type: doc.type as "file" | "folder" })
        }
      })
    }

    return references
  }

  const handleSendMessage = () => {
    if (!inputValue.trim()) return

    const references = extractReferences(inputValue)
    const newMessage: Message = {
      id: messages.length + 1,
      type: "user",
      content: inputValue,
      timestamp: new Date(),
      references: references.length > 0 ? references : undefined,
    }

    setMessages([...messages, newMessage])
    setInputValue("")
    setShowAutocomplete(false)

    // Simulate AI response
    setIsTyping(true)
    setTimeout(() => {
      const aiResponse: Message = {
        id: messages.length + 2,
        type: "ai",
        content: generateAIResponse(inputValue, selectedPersonality, references),
        timestamp: new Date(),
        personality: selectedPersonality,
      }
      setMessages((prev) => [...prev, aiResponse])
      setIsTyping(false)
    }, 2000)
  }

  const generateAIResponse = (userMessage: string, personality: string, references: DocumentReference[]) => {
    const responses = {
      professional:
        "I understand your request. Based on the information provided and the referenced documents, here's my analysis:",
      technical: "Let me break down the technical aspects of your query. From a systems perspective:",
      casual: "Hey! I'd be happy to help with that. Looking at what you've shared:",
    }

    const baseResponse = responses[personality as keyof typeof responses]

    if (references.length > 0) {
      return `${baseResponse}\n\nI can see you've referenced ${references.map((r) => r.name).join(", ")}. I'll analyze these documents and provide you with detailed insights shortly.`
    }

    return `${baseResponse}\n\nCould you provide more specific details about what you'd like me to help you with?`
  }

  const filteredMessages = messages.filter(
    (message) => searchQuery === "" || message.content.toLowerCase().includes(searchQuery.toLowerCase()),
  )

  return (
    <div className="flex h-[calc(100vh-4rem)] gap-6 p-6">
      {/* Documents Sidebar */}
      <Card className="w-80 flex flex-col">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <FileText className="h-5 w-5" />
            Documents
          </CardTitle>
          <div className="relative">
            <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
            <Input placeholder="Search documents..." className="pl-8" />
          </div>
        </CardHeader>
        <CardContent className="flex-1 p-0">
          <ScrollArea className="h-full px-4">
            <div className="space-y-2 pb-4">
              {mockDocuments.map((doc) => (
                <div
                  key={doc.id}
                  className="flex items-center gap-2 p-2 rounded hover:bg-muted cursor-pointer"
                  onClick={() => setSelectedDocument(doc)}
                >
                  {doc.type === "folder" ? <Folder className="h-4 w-4" /> : <FileText className="h-4 w-4" />}
                  <span className="text-sm truncate">{doc.name}</span>
                </div>
              ))}
            </div>
          </ScrollArea>
        </CardContent>
      </Card>

      {/* Chat Interface */}
      <Card className="flex-1 flex flex-col">
        <CardHeader className="border-b">
          <div className="flex items-center justify-between">
            <CardTitle className="flex items-center gap-2">
              <Bot className="h-5 w-5" />
              AI Assistant
            </CardTitle>
            <div className="flex items-center gap-4">
              <div className="relative">
                <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
                <Input
                  placeholder="Search messages..."
                  className="pl-8 w-64"
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                />
              </div>
              <Select value={selectedPersonality} onValueChange={setSelectedPersonality}>
                <SelectTrigger className="w-40">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {personalities.map((personality) => (
                    <SelectItem key={personality.id} value={personality.id}>
                      {personality.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          </div>
        </CardHeader>

        <CardContent className="flex-1 flex flex-col p-0 min-h-0">
          <ScrollArea className="flex-1 p-6">
            <div className="space-y-4 min-w-0">
              {filteredMessages.map((message) => (
                <MessageComponent key={message.id} message={message} onDocumentClick={setSelectedDocument} />
              ))}
              {isTyping && (
                <div className="flex gap-3">
                  <div className="w-8 h-8 rounded-full bg-primary flex items-center justify-center">
                    <Bot className="h-4 w-4 text-primary-foreground" />
                  </div>
                  <div className="bg-muted rounded-lg px-4 py-2">
                    <div className="flex space-x-1">
                      <div className="w-2 h-2 bg-muted-foreground rounded-full animate-bounce"></div>
                      <div
                        className="w-2 h-2 bg-muted-foreground rounded-full animate-bounce"
                        style={{ animationDelay: "0.1s" }}
                      ></div>
                      <div
                        className="w-2 h-2 bg-muted-foreground rounded-full animate-bounce"
                        style={{ animationDelay: "0.2s" }}
                      ></div>
                    </div>
                  </div>
                </div>
              )}
              <div ref={messagesEndRef} />
            </div>
          </ScrollArea>

          <Separator />

          <div className="p-4">
            <div className="relative">
              <DocumentAutocomplete
                isOpen={showAutocomplete}
                onSelect={handleDocumentSelect}
                onClose={() => setShowAutocomplete(false)}
                searchTerm={autocompleteSearch}
              />
              <div className="flex gap-2">
                <div className="relative flex-1">
                  <Input
                    ref={inputRef}
                    placeholder="Type @ to reference documents..."
                    value={inputValue}
                    onChange={(e) => handleInputChange(e.target.value)}
                    onKeyPress={(e) => e.key === "Enter" && handleSendMessage()}
                    className="pr-10"
                  />
                  <Button variant="ghost" size="sm" className="absolute right-1 top-1 h-8 w-8 p-0">
                    <Paperclip className="h-4 w-4" />
                  </Button>
                </div>
                <Button onClick={handleSendMessage} disabled={!inputValue.trim()}>
                  <Send className="h-4 w-4" />
                </Button>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Document Preview Modal */}
      <DocumentPreviewModal
        document={selectedDocument}
        isOpen={!!selectedDocument}
        onClose={() => setSelectedDocument(null)}
      />
    </div>
  )
}
