import { useState } from "react";
import CryptoJS from "crypto-js";

const Login = () => {
    const [username,setUsername] = useState("")
    const [password, setPassword] = useState("")

    const handleLogin = () => {
        console.log("login")
        // axios.post()
    //     fetch("http://localhost:5000/login", {
    //         method: "POST",
    //         headers: {
    //             "Content-Type": "application/json"
    //         },
    //         body: JSON.stringify({ email, hashedPassword })
    //     })
    //         .then(res => {
    //             if (res.ok) {
    //                 return res.json()
    //             } else {
    //                 return res.json().then((data) => {
    //                     console.log(data.msg)
    //                 })
    //             }
    //         })
    //         .then(data => {
    //             console.log(data.token);
    //             localStorage.setItem("jwtToken", data.token);
    //             setLoggedIn(true)
    //             navigate("/")
    //         })
    //         .catch(e => {
    //             console.log('Login error:', e)
    //         })
    // }
    }
    return ( 
        <div className="login-form">
            <h1>LOGIN</h1>
            <input className="login-input" type="text" name="username" placeholder="username" onChange={e => setUsername(e.target.value)}/>
            <input className="login-input" type="password" name="hashedPassword" placeholder="password" onChange={e => setPassword(CryptoJS.SHA256(e.target.value).toString())} />
            <button className="log-btn signin" onClick={handleLogin}>SIGN IN</button>
            {username}
            {password}
        </div>
     );
}
 
export default Login;