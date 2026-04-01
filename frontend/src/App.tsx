import { useState, useEffect, useRef, useCallback } from 'react'
import PromptEditor, { GenerateParams } from './components/PromptEditor'
import ResponseEditor from './components/ResponseEditor'
import ResponseViewer from './components/ResponseViewer'

const DEFAULT_MODELS = ['llama3.2', 'mistral', 'codellama']

const DEFAULT_PARAMS: GenerateParams = {
  model: DEFAULT_MODELS[0],
  maxTokens: 512,
  temperature: 0.7,
  topP: 1.0,
}

interface Toast {
  message: string
  type: 'success' | 'error'
}

export default function App() {
  const [prompt, setPrompt] = useState('')
  const [response, setResponse] = useState('')
  const [editedResponse, setEditedResponse] = useState('')
  const [params, setParams] = useState<GenerateParams>(DEFAULT_PARAMS)
  const [isStreaming, setIsStreaming] = useState(false)
  const [isLoading, setIsLoading] = useState(false)
  const [models, setModels] = useState<string[]>(DEFAULT_MODELS)
  const [toast, setToast] = useState<Toast | null>(null)
  const wsRef = useRef<WebSocket | null>(null)

  useEffect(() => {
    fetchModels()
  }, [])

  const fetchModels = async () => {
    try {
      const res = await fetch('/api/models')
      if (res.ok) {
        const data = await res.json()
        if (data.models && Array.isArray(data.models)) {
          setModels(data.models)
        }
      }
    } catch {
      console.log('Using default models')
    }
  }

  const showToast = (message: string, type: 'success' | 'error') => {
    setToast({ message, type })
    setTimeout(() => setToast(null), 3000)
  }

  const stopGeneration = useCallback(() => {
    if (wsRef.current) {
      wsRef.current.close()
      wsRef.current = null
    }
    setIsStreaming(false)
    setIsLoading(false)
  }, [])

  const generate = useCallback(() => {
    if (!prompt.trim()) return

    setResponse('')
    setEditedResponse('')
    setIsStreaming(true)
    setIsLoading(true)

    const ws = new WebSocket(`ws://${window.location.host}/ws/generate`)
    wsRef.current = ws

    ws.onopen = () => {
      ws.send(JSON.stringify({
        prompt,
        model: params.model,
        max_tokens: params.maxTokens,
        temperature: params.temperature,
        top_p: params.topP,
      }))
    }

    ws.onmessage = (event) => {
      const data = event.data
      if (data.startsWith('__ERROR__:')) {
        showToast(data.replace('__ERROR__:', ''), 'error')
        stopGeneration()
        return
      }
      try {
        const parsed = JSON.parse(data)
        if (parsed.text) {
          setResponse(prev => prev + parsed.text)
        }
        if (parsed.done) {
          setIsStreaming(false)
          setIsLoading(false)
          ws.close()
        }
      } catch {
        setResponse(prev => prev + data)
      }
    }

    ws.onerror = () => {
      showToast('Connection error', 'error')
      stopGeneration()
    }

    ws.onclose = () => {
      setIsStreaming(false)
      setIsLoading(false)
    }
  }, [prompt, params, stopGeneration])

  const handleEditorApply = (content: string) => {
    setEditedResponse(content)
    showToast('Applied to preview', 'success')
  }

  const handleAccept = () => {
    const finalResponse = editedResponse || response
    setPrompt(prev => prev + '\n\n' + finalResponse)
    showToast('Response accepted', 'success')
  }

  const handleEdit = (edited: string) => {
    setResponse(edited)
    setEditedResponse(edited)
    showToast('Response updated', 'success')
  }

  return (
    <div className="app">
      <header className="header">
        <h1>non-Turbo Ollama</h1>
        <div className="status">
          {isStreaming ? (
            <span className="status-streaming">Streaming...</span>
          ) : isLoading ? (
            <span>Loading...</span>
          ) : (
            <span>Ready</span>
          )}
        </div>
      </header>

      <main className="main-content">
        <div className="left-column">
          <PromptEditor
            value={prompt}
            onChange={setPrompt}
            onGenerate={generate}
            onStop={stopGeneration}
            params={params}
            onParamsChange={setParams}
            models={models}
            isStreaming={isStreaming}
            isLoading={isLoading}
          />

          <ResponseViewer
            content={response}
            onAccept={handleAccept}
            onEdit={handleEdit}
            isStreaming={isStreaming}
          />
        </div>

        <ResponseEditor
          content={editedResponse || response}
          onSave={handleEditorApply}
          onCopy={() => navigator.clipboard.writeText(editedResponse || response)}
          isStreaming={isStreaming}
        />
      </main>

      {toast && (
        <div className={`toast ${toast.type}`}>
          {toast.message}
        </div>
      )}
    </div>
  )
}
