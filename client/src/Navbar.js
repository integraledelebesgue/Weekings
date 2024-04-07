import {NavLink} from "react-router-dom";
import "./styles/navbar.css";
import useAuth from "./hooks/useAuth";

const Navbar = () => {

    const {auth,setAuth} = useAuth();

    const handleLogout = () => {
        setAuth(false);
    }
    
    return ( 
        <nav className="navbar">
            <h2 className="logo">Weekings</h2>
            { auth && <div className="middle-links">
                <NavLink to="/">Main page</NavLink>
                <NavLink to="/">Friends</NavLink>
                <NavLink to="/groups">Groups</NavLink>
            </div> }
            {auth && <button onClick={e => {handleLogout()}} className="login-link">Login</button>}
        </nav>
     );
}
 
export default Navbar;