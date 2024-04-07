import API from "./api";
import useFetch from "./hooks/useFetch";

const Feed = () => {

    const {data: posts, isPending, error} = useFetch(API, "/feed");

    return ( 
        <div>
            {error && <h2>Couldn't fetch info</h2>}
            {isPending && <p>loading...</p>} 
            {posts && posts.map(post => {
                <div className="post" key={post.id}>
                    <h3>{post.text}</h3>
                    <button>Click</button>
                </div>
            })}
        </div>
     );
}
 
export default Feed;