import API from "./api";
import useFetch from "./hooks/useFetch";

const Friends = () => {

    const {data: friendlist, isPending, error} = useFetch(API,"/friends");
    return ( 
        <div className="container">
            {error && <p>Error</p>}
            {isPending && <p>Loading...</p>}
            <button>Add new friend</button>
            {friendlist && 
                friendlist.map(friend => {
                    <div>
                        {friend.name}
                    </div>
                })}
        </div>
     );
}
 
export default Friends;