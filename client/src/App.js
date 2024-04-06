import logo from './logo.svg';
import './App.css';
import {BrowserRouter, Routes, Route} from "react-router-dom"
import Groups from './Groups';
import Login from './LoginPage';
function App() {
  return (
    <div className="App">
      <BrowserRouter></BrowserRouter>
      <Groups/>
      <Login/>
    </div>
  );
}

export default App;
