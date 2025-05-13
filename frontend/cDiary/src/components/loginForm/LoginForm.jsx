import './LoginForm.css';
import { useState } from 'react';
import { loadDiary } from '../../api/diary';
import { useNavigate } from 'react-router-dom';


function LoginForm(){
    const [diaryName, setDiaryName] = useState("");
    const [diaryAddress, setDiaryAddress] = useState("");
    const [error, setError] = useState("");
    const Navigate = useNavigate();
    const handleSubmit = async (e) => {
        e.preventDefault();
        setError("");
        
        try{
            const data = await loadDiary({name: diaryName, address: diaryAddress});
            console.log(data);

            Navigate(`/entries/${data}`);
        }
        catch(err){
            setError("Failed to load diary");
            console.error(err);
        }
    }

    return(
        <div className="loginForm">
            <div className="frmContainer">
                <h2 style={{paddingTop: 0}}>Access your diary</h2> 
                <form className="frm" onSubmit={handleSubmit}>
                    <div id='submitHolder'>
                        <input className='formInput' type="text" placeholder="Diary name" onChange={(e) => setDiaryName(e.target.value)} required />
                        <input className='formInput' type="text" placeholder="Diary address" onChange={(e) => setDiaryAddress(e.target.value)} required />
                        <input id='b1' type="submit" value="Login" />
                    </div>
                </form>
            </div>
            <div className='signup'>
                <p id='smalltxt'>Be sure to not forget your diary name and address</p>
            </div>
        </div>
    );
}

export default LoginForm;