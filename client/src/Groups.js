// component for displaying list of Groups

// TODO:
// fetching groups from DB
//
const Groups = () => {

    const groups = [1,2,3,4,5,6]
    return (
        <div>
            <h1>Hello</h1> 
        {groups.map(group => {
            console.log(group);
            <div>group
            <p>hello</p></div>
        })}
        </div>
     );
}
 
export default Groups;