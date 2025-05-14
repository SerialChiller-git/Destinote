import Home from './components/home/home'
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom'
import './App.css'
import DiaryPage from './components/diaryPage/DiaryPage'

function App() {
  return (
      <Router>
        <Routes>
          <Route path='/' element={<Home />} />
          <Route path='/entries/:id' element={<DiaryPage />} />
\        </Routes>
      </Router>
  )
}

export default App
