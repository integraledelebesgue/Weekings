import { useState } from "react";
import CryptoJS from "crypto-js";
import useAuth from './hooks/useAuth';
import API from "./api";
import "./styles/login.css";

const Login = () => {

    const [username,setUsername] = useState('');
    const [password,setPassword] = useState('');
    const [error, setError] = useState('');
    const {auth, setAuth} = useAuth();

    const handleLogin = async (e) => {
        e.preventDefault();
        try {
            const res = await API.post("/login", {
            username,
            password,
            }).then((res) => {
            if (res.data === 202) {
                setAuth(true);
                setUsername("");
                setPassword("");
                console.log("You are logged in")
            } else {
                console.log("incorrect submission");
                setError(res.message);
            }
            });
        } catch (err) {
            if (!err?.response) {
            setError("no server response");
            } else {
            setError("authentication failed");
            }
        }
    }

    return (
        <div className="container">
            <div className="login-form">
                <h1>LOGIN</h1>
                <input className="login-input" type="text" name="username" placeholder="username" onChange={e => setUsername(e.target.value)}/>
                <input className="login-input" type="password" name="hashedPassword" placeholder="password" onChange={e => setPassword(CryptoJS.SHA256(e.target.value).toString())} />
                <button className="btn signin" onClick={handleLogin}>SIGN IN</button>
 
                <hr />
                <h1 className="register">Don't have an account?<br /> Register right now</h1>
                <button className="register btn">Register</button>
            </div>
            <br />
        </div> 
     );

}
 
export default Login;