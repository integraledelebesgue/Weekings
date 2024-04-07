import axios from "axios";

const API = axios.create({
    baseURL: "https://weeking.shuttleapp.rs",
    withCredentials: true,
  });

  export default API;