import React, { useState, useEffect } from "react";

function Todos({reloadData, setReloadData}) {
  const [todos, setTodos] = useState(null);
  const baseUrl = process.env.REACT_APP_BASE_URL || "http://localhost:8080";
  useEffect(() => {
    if(todos === null  || reloadData) {
      fetch(`${baseUrl}/todos`)
      .then((response) => response.json())
      .then((data) => setTodos(data));
      setReloadData(false);
    }

  }, [baseUrl,todos, reloadData]);

  async function handleDelete(event, id) {
    event.preventDefault();
    let resp = await fetch(`${baseUrl}/delete-todo/${id}`, {
      method: "delete",
    });
    let serverResp = await resp.json();
    console.log(serverResp.result);
    setReloadData(true);

  }

  return (
      <table className="table table-bordered table-responsive">
        <thead>
          <tr>
            <th>#</th>
            <th>Todo</th>
            <th>Date Creation</th>
            <th>Action</th>
          </tr>
        </thead>
        <tbody>
          {!todos?.length ? (
            <tr>
              <td colSpan="4">Nothing to do</td>
            </tr>
          ) : (
            todos.map((t) => {
              return (
                <tr key={t.id}>
                  <td>{t.id}</td>
                  <td>{t.descr}</td>
                  <td>{t.date_created}</td>
                  <td>
                    <button className="btn btn-danger" onClick={(e) => handleDelete(e, t.id)}>Delete</button>
                  </td>
                </tr>
              );
            })
          )}
        </tbody>
      </table>
  
  );
}
export default Todos;
