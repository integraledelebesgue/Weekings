import API from "./api";
import useFetch from "./hooks/useFetch";

const Profile = () => {

    const {data:profileInfo, isPending, error} = useFetch(API,"/profile");

    return ( 
        <div>
            
            <h1>Hello {}</h1>
        </div>
     );
}
 
export default Profile;