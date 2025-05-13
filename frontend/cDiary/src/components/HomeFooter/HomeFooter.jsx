import './HomeFooter.css';

function HomeFooter(){
    return(
        <footer className="footer">
        <div className="footer-content">
          <div className="heart">❤️</div>
          <p className="footer-text">Made with love by us — our little world of memories 💖</p>
          <p className="footer-copy">© {new Date().getFullYear()} <b>cDiary</b></p>
        </div>
      </footer>
    );
}

export default HomeFooter;