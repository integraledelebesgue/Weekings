import API from "./api";
import useFetch from "./hooks/useFetch";
import { useNavigate } from "react-router-dom";

const Groups = () => {

    const {data: groups, isPending, error} = useFetch(API,"/groups")
    const navigate = useNavigate();

    const handleGoToGroup = (id) => {
        navigate(`/groups/${id}`);
    }
    return (
        <div className="groups-container">
            <h1>Your groups</h1>
        {error && <h2>Couldn't fetch info</h2>}
        {isPending && <p>loading...</p>} 
        {groups && groups.map(group => {
            <div className="group" key={group.id}>
                <h3>{group.name}</h3>
                <button onClick={e => {handleGoToGroup(group.id)}}>Click</button>
            </div>
        })}
        </div>
     );
}
 
export default Groups;