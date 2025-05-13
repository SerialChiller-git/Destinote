export const loadDiary = async (data) => {
    const response = await fetch("http://localhost:8080/api/create", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            name : data.name,
            address : data.address
        })
    });
    if(!response.ok){
        throw new Error("Failed to fetch diary");
    }
    else{
        return await response.json();
    }
}