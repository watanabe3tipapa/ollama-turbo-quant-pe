import { useState, useEffect, useRef } from 'react'
import Editor, { OnMount } from '@monaco-editor/react'
import type { editor } from 'monaco-editor'

interface ResponseEditorProps {
  content: string
  onSave: (content: string) => void
  onCopy: () => void
  isStreaming: boolean
}

export default function ResponseEditor({
  content,
  onSave,
  onCopy,
  isStreaming,
}: ResponseEditorProps) {
  const [editedContent, setEditedContent] = useState(content)
  const [isWordWrap, setIsWordWrap] = useState(true)
  const [isMinimap, setIsMinimap] = useState(false)
  const [fontSize, setFontSize] = useState(14)
  const editorRef = useRef<editor.IStandaloneCodeEditor | null>(null)

  useEffect(() => {
    setEditedContent(content)
  }, [content])

  const handleEditorMount: OnMount = (editor) => {
    editorRef.current = editor
  }

  const toggleWordWrap = () => {
    if (editorRef.current) {
      const newValue = !isWordWrap
      editorRef.current.updateOptions({ wordWrap: newValue ? 'on' : 'off' })
      setIsWordWrap(newValue)
    }
  }

  const toggleMinimap = () => {
    if (editorRef.current) {
      const newValue = !isMinimap
      editorRef.current.updateOptions({ minimap: { enabled: newValue } })
      setIsMinimap(newValue)
    }
  }

  const increaseFontSize = () => {
    if (editorRef.current) {
      const newSize = fontSize + 1
      editorRef.current.updateOptions({ fontSize: newSize })
      setFontSize(newSize)
    }
  }

  const decreaseFontSize = () => {
    if (editorRef.current) {
      const newSize = Math.max(10, fontSize - 1)
      editorRef.current.updateOptions({ fontSize: newSize })
      setFontSize(newSize)
    }
  }

  const formatDocument = () => {
    if (editorRef.current) {
      editorRef.current.getAction('editor.action.formatDocument')?.run()
    }
  }

  const undoChange = () => {
    if (editorRef.current) {
      editorRef.current.getAction('undo')?.run()
    }
  }

  const redoChange = () => {
    if (editorRef.current) {
      editorRef.current.getAction('redo')?.run()
    }
  }

  const selectAll = () => {
    if (editorRef.current) {
      editorRef.current.setSelection(
        editorRef.current.getModel()?.getFullModelRange()!
      )
    }
  }

  const saveToFile = () => {
    const blob = new Blob([editedContent], { type: 'text/markdown' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `response_${Date.now()}.md`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  }

  return (
    <div className="panel">
      <div className="panel-header">
        <span className="panel-title">Editor</span>
        <div className="editor-toolbar">
          <button
            className={`toolbar-btn ${isWordWrap ? 'active' : ''}`}
            onClick={toggleWordWrap}
            title="Word Wrap"
          >
            Wrap
          </button>
          <button
            className={`toolbar-btn ${isMinimap ? 'active' : ''}`}
            onClick={toggleMinimap}
            title="Minimap"
          >
            Map
          </button>
          <button
            className="toolbar-btn"
            onClick={increaseFontSize}
            title="Increase Font Size"
          >
            A+
          </button>
          <button
            className="toolbar-btn"
            onClick={decreaseFontSize}
            title="Decrease Font Size"
          >
            A-
          </button>
          <button
            className="toolbar-btn"
            onClick={formatDocument}
            title="Format Document"
          >
            Fmt
          </button>
          <button
            className="toolbar-btn"
            onClick={undoChange}
            title="Undo"
          >
            Undo
          </button>
          <button
            className="toolbar-btn"
            onClick={redoChange}
            title="Redo"
          >
            Redo
          </button>
          <button
            className="toolbar-btn"
            onClick={selectAll}
            title="Select All"
          >
            All
          </button>
        </div>
        <div className="panel-actions">
          <button className="btn btn-sm btn-secondary" onClick={onCopy}>
            Copy
          </button>
          <button
            className="btn btn-sm btn-secondary"
            onClick={saveToFile}
            disabled={!editedContent.trim() || isStreaming}
          >
            Save
          </button>
          <button
            className="btn btn-sm btn-primary"
            onClick={() => onSave(editedContent)}
            disabled={!editedContent.trim() || isStreaming}
          >
            Apply
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
          onMount={handleEditorMount}
          options={{
            wordWrap: isWordWrap ? 'on' : 'off',
            minimap: { enabled: isMinimap },
            fontSize: fontSize,
            lineNumbers: 'on',
            folding: true,
            padding: { top: 16 },
            readOnly: isStreaming,
            scrollBeyondLastLine: false,
            automaticLayout: true,
            tabSize: 2,
            renderWhitespace: 'selection',
            quickSuggestions: true,
            suggestOnTriggerCharacters: true,
          }}
        />
      </div>
      <div className="panel-footer">
        <div className="token-count">
          {editedContent.length} chars | Font: {fontSize}px
        </div>
      </div>
    </div>
  )
}
