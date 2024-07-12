import React from 'react'
import ReactDOM from 'react-dom/client'
import Home from '@/app/page'
import './globals.css'
import { MainLayout } from './app/main-layout'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <MainLayout>
      <Home />
    </MainLayout>
  </React.StrictMode>,
)
