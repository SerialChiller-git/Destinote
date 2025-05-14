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

export const loadEntries = async (id) => {
    const response = await fetch(`http://localhost:8080/api/diary/${id}`,
        {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
            }
        }
    );
    if(!response.ok){
        throw new Error("Failed to fetch entries");
    }
    else{
        const data = await response.json();
        const realData = data.Ok;
        console.log(realData);
        return realData;
    }
}

export const saveEntry = async (id, entry) => {
    const response = await fetch(`http://localhost:8080/api/diary/${id}`, {
        method: "PUT",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(
            entry
        ),
    })
}