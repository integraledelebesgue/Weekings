import logo from './logo.svg';
import './App.css';
import {BrowserRouter as Router, Routes, Route} from "react-router-dom"
import Groups from './Groups';
import Login from './LoginPage';
import { AuthProvider } from './context/AuthContext';
import Hello from './Hello';
function App() {
  return (
    <Router>
      <div className="App">
        <AuthProvider>
          <Routes>
            <Route path="/" element={<Hello />} />
            <Route path="/login" element={<Login/>} />
          </Routes>
        </AuthProvider>
      </div>
    </Router>
  );
}

export default App;
