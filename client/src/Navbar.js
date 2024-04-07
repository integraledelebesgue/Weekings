import {NavLink} from "react-router-dom";
import "./styles/navbar.css";
const Navbar = () => {
    return ( 
        <nav className="navbar">
            <h2 className="logo">Weekings</h2>
            <div className="middle-links">
                <NavLink to="/">Main page</NavLink>
                <NavLink to="/">Friends</NavLink>
                <NavLink to="/groups">Groups</NavLink>
            </div>
            <NavLink to="/login">Login</NavLink>
        </nav>
     );
}
 
export default Navbar;