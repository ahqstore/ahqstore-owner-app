import { Button } from "@/components/ui/button";
import "./App.css";

function App() {
  return (
    <div className="container">
      <div className="curved-box">
        <div className="option active">
          <a href="#">Dashboard</a>
        </div>
        <div className="option active">
          <a href="#">Settings</a>
        </div>
        <div className="option active">
          <a href="#">Help</a>
        </div>
      </div>
    </div>
  );
}

export default App;