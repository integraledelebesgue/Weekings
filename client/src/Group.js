import {useParams, useNavigate} from "react-router-dom";
import useFetch from "./hooks/useFetch";
import API from "./api";

const Group = () => {
    const {id} = useParams();
    const {data: group, isPending:pendingGroup, error:groupError} = useFetch(API,`/groups/${id}`);
    const {data: posts, isPending: pendingPosts, error: postError} = useFetch(API,`groups/${id}/posts`);
    
    return ( 
        <div>
            {groupError && <h1>Sorry, something went wrong</h1>}
            {pendingGroup && <h2>Loading...</h2>}
            {group && <div className="group-container">
                <div>Here is div for current avaialable quests</div>
                <div className="posts-wall">
                    <h2>Posts are displayed here</h2>
                    {postError && <p>Couldn't fetch posts</p>}
                    {pendingPosts && <p>Loading...</p>}
                    {posts && 
                    <div> {posts.map(post => {
                        <div className="post" key={post.id}>
                            <p>{post.creation_date}</p>
                            <p>{post.text}</p>
                        </div>
                    })}</div>}
                </div>
                </div>}
        </div>
     );
}
 
export default Group;