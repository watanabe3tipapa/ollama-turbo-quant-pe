import { useState, useCallback, useRef, useEffect } from 'react'
import Editor from '@monaco-editor/react'

export interface GenerateParams {
  model: string
  maxTokens: number
  temperature: number
  topP: number
}

interface PromptEditorProps {
  value: string
  onChange: (value: string) => void
  onGenerate: () => void
  onStop: () => void
  params: GenerateParams
  onParamsChange: (params: GenerateParams) => void
  models: string[]
  isStreaming: boolean
  isLoading: boolean
}

export default function PromptEditor({
  value,
  onChange,
  onGenerate,
  onStop,
  params,
  onParamsChange,
  models,
  isStreaming,
  isLoading,
}: PromptEditorProps) {
  return (
    <div className="panel">
      <div className="panel-header">
        <span className="panel-title">Prompt</span>
        <div className="panel-actions">
          <button
            className="btn btn-sm btn-secondary"
            onClick={() => onChange('')}
            disabled={!value.trim() || isStreaming}
          >
            Clear
          </button>
          <select
            className="model-select"
            value={params.model}
            onChange={(e) => onParamsChange({ ...params, model: e.target.value })}
          >
            {models.map((m) => (
              <option key={m} value={m}>{m}</option>
            ))}
          </select>
        </div>
      </div>
      <div className="panel-body">
        <Editor
          height="100%"
          defaultLanguage="markdown"
          theme="vs-dark"
          value={value}
          onChange={(v) => onChange(v || '')}
          options={{
            minimap: { enabled: false },
            fontSize: 14,
            lineNumbers: 'off',
            folding: false,
            wordWrap: 'on',
            padding: { top: 16 },
          }}
        />
      </div>
      <div className="panel-footer">
        <div className="params-row">
          <div className="param-group">
            <label className="param-label">Max Tokens</label>
            <input
              type="number"
              className="param-input"
              value={params.maxTokens}
              onChange={(e) => onParamsChange({ ...params, maxTokens: parseInt(e.target.value) || 150 })}
            />
          </div>
          <div className="param-group">
            <label className="param-label">Temperature</label>
            <input
              type="number"
              className="param-input"
              step="0.1"
              min="0"
              max="2"
              value={params.temperature}
              onChange={(e) => onParamsChange({ ...params, temperature: parseFloat(e.target.value) || 0.7 })}
            />
          </div>
          <div className="param-group">
            <label className="param-label">Top P</label>
            <input
              type="number"
              className="param-input"
              step="0.1"
              min="0"
              max="1"
              value={params.topP}
              onChange={(e) => onParamsChange({ ...params, topP: parseFloat(e.target.value) || 1 })}
            />
          </div>
        </div>
        {isStreaming ? (
          <button className="btn btn-danger" onClick={onStop}>
            <span className="loading-spinner" />
            Stop
          </button>
        ) : (
          <button
            className="btn btn-primary"
            onClick={onGenerate}
            disabled={!value.trim() || isLoading}
          >
            Generate
          </button>
        )}
      </div>
    </div>
  )
}
