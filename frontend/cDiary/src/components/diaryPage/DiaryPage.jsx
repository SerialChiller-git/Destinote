import './DiaryPage.css';
import { useParams } from 'react-router-dom';
import { useEffect, useState } from 'react';
import { loadEntries, saveEntry } from '../../api/diary';

function DiaryPage(){
    const {id}= useParams();
    const [entry , setEntry] = useState("");
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const load = async () => {
            try{
                const fetchedEntry = await loadEntries(id);
                setEntry(fetchedEntry);
            }
            catch(err){
                console.error("Failed to load entries", err);
            }
            finally{
                setLoading(false);
            }
        };
        load();
    }, [id]);

    const handleSave = async () => {
        try{
            await saveEntry(id, entry);
            alert("Entry saved successfully");
        }
        catch(err){
            console.error("Failed to save entry", err);
        }
    }


    return(
        <div className='diaryPage'>
            <div className='field'>
                <textarea className='diaryText' 
                    value={entry}
                    onChange={(e) => setEntry(e.target.value)}
                    placeholder='Write your diary here...' 
                />
                <button onClick={handleSave}> Save </button>
            </div>
        </div>
    );
}


export default DiaryPage;