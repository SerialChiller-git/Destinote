import './home.css';
import LoginForm from '../loginForm/LoginForm.jsx';
import HomeFooter from '../HomeFooter/HomeFooter.jsx';

function Home(){
    return(
        <div className='page'>
            <div className="headerBar">
                <h1>cDiary</h1>
            </div>
            <LoginForm />
            <HomeFooter /> 
        </div>
    );
}

export default Home;