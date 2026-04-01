import { useState, useEffect } from 'react'
import Editor from '@monaco-editor/react'

interface ResponseViewerProps {
  content: string
  onAccept: () => void
  onEdit: (edited: string) => void
  isStreaming: boolean
}

export default function ResponseViewer({
  content,
  onAccept,
  onEdit,
  isStreaming,
}: ResponseViewerProps) {
  const [editedContent, setEditedContent] = useState(content)

  useEffect(() => {
    setEditedContent(content)
  }, [content])

  const handleCopy = () => {
    navigator.clipboard.writeText(editedContent)
  }

  return (
    <div className="panel">
      <div className="panel-header">
        <span className="panel-title">
          Preview {isStreaming && <span className="status-streaming">(streaming)</span>}
        </span>
        <div className="panel-actions">
          <button className="btn btn-sm btn-secondary" onClick={handleCopy}>
            Copy
          </button>
        </div>
      </div>
      <div className="panel-body">
        <Editor
          height="100%"
          defaultLanguage="markdown"
          theme="vs-dark"
          value={editedContent}
          onChange={(v) => setEditedContent(v || '')}
          options={{
            minimap: { enabled: false },
            fontSize: 14,
            lineNumbers: 'off',
            folding: false,
            wordWrap: 'on',
            padding: { top: 16 },
            readOnly: isStreaming,
          }}
        />
      </div>
      <div className="panel-footer">
        <div className="token-count">
          {content.length} chars
        </div>
        <div className="panel-actions">
          <button className="btn btn-secondary" onClick={() => onEdit(editedContent)}>
            Edit & Re-apply
          </button>
          <button
            className="btn btn-success"
            onClick={onAccept}
            disabled={!content.trim() || isStreaming}
          >
            Accept
          </button>
        </div>
      </div>
    </div>
  )
}
