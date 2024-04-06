import { useEffect,useState } from "react";
import axios from "axios";

const useFetch = (api,url) => {
    const [data, setData] = useState(null);
    const [isPending,setIsPending] = useState(true);
    const [error, setError] = useState(null);

    useEffect(() => {
        const source = axios.CancelToken.source();

        api
        .get(url,{cancelToken:source.token})
        .then((res)=>{
            if(res.status !== 200){
                throw Error('could not fetch the data');
            }
            return res.data;
        })
        .then((data)=>{
            setData(data);
            setIsPending(false);
            setError(null);
        })
        .catch((err)=>{
            if (axios.isCancel(err)) {
                console.log("Request canceled:", err.message);
              } else {
                setError(err.message);
                setIsPending(false);
              }
        })
        
        return () => source.cancel("Request canceled");
    },[api, url]);
    
    return {data, isPending, error};
}
export default useFetch;