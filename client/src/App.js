import logo from './logo.svg';
import './App.css';
import {BrowserRouter as Router, Routes, Route, Navigate} from "react-router-dom"
import Groups from './Groups';
import Login from './LoginPage';
import { AuthProvider } from './context/AuthContext';
import Hello from './Hello';
import useAuth from './hooks/useAuth';
import Group from './Group';
import Navbar from './Navbar';
import Friends from './Friends';
function App() {

  const PrivateRoute = ({ element }) => {
    const { auth } = useAuth();
    return auth ? element : <Navigate to="/login" />;
  };

  return (
    <Router>
      <div className="App">
        <AuthProvider>
          <Navbar/>
          <Routes>
            <Route path="/" element={<Hello/>}/>
            <Route path="/login" element={<Login/>}/>
            <Route path="/groups" element={<PrivateRoute element={<Groups/>}/>}/>
            <Route path="/groups:id" element={<PrivateRoute element={<Group/>}/>}></Route>
            {/* <Route path="/friends" element={<PrivateRoute element={<Friends/>}/>}></Route> */}
          </Routes>
        </AuthProvider>
      </div>
    </Router>
  );
}

export default App;
