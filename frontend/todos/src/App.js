import "./App.css";
import Todos from "./components/Todos";
import NewTodo from "./components/NewTodo";
import React, { useState, useEffect } from "react";

function App() {
  const [reloadData, setReloadData] = useState(false);

  return (
    <div className="container">
      <div className="mt-2">
        <NewTodo setReloadData={setReloadData} />
      </div>
      <div className=" mt-2">
      <Todos reloadData={reloadData} setReloadData={setReloadData} />
      </div>
    </div>
  );
}

export default App;
