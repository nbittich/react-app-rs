import React, { useState } from "react";
function NewTodo({setReloadData}) {
  const baseUrl = process.env.REACT_APP_BASE_URL || "http://localhost:8080";
  const [descr, setDescr] = useState("");
  const [responseMessage, setResponseMessage] = useState(null);
  const handleSubmit = (e) => {
    e.preventDefault();
    submitNewTodo();
  };
  async function submitNewTodo() {
    const data = new URLSearchParams();
    data.append("descr", descr);

    let resp = await fetch(`${baseUrl}/new-todo`, {
      method: "post",
      body: data,
    });
    let todo = await resp.json();
    setResponseMessage(`todo with descr ${todo.descr} and id ${todo.id} inserted`);
    setTimeout(()=> setResponseMessage(null), 2000);
    setDescr('');
    setReloadData(true);
  }
  return (
    <div className="card">
      <div className="card-header">New todo</div>
      <div className="card-body">
        <form className="mt-2" onSubmit={handleSubmit}>
          <div className="mb-3">
            <label htmlFor="descr" className="form-label">
              Todo
            </label>
            <textarea
              onChange={(event) => setDescr(event.target.value)}
              value={descr}
              placeholder="Your todo here"
              className="form-control"
              id="descr"
              name="descr"
              required
              min="3"
            ></textarea>

            <div id="descrHelp" className="form-text">
              Add something useful.
            </div>
          </div>
          <button type="submit" className="btn btn-primary">
            Submit
          </button>
        </form>
        {responseMessage ? 
       <div className="alert mt-2 alert-primary">
         {responseMessage}
       </div> : '' 
      }
       
      </div>
    </div>
  );
}
export default NewTodo;
