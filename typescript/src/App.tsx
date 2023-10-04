import React, { useEffect, useRef, useState } from 'react';

interface Message {
  User?: string;
  Assistant?: [string, string[]];
}

interface Conversation extends Array<Message> {};

function App() {
  const [inputText, setInputText] = useState<string>('');
  const [conversation, setConversation] = useState<Conversation>([]);
  const messagesEndRef = useRef<null | HTMLDivElement>(null);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' })
  }, [conversation]);

  async function submit() {
    const userMessageArray = [{ User: inputText }];

    // First we add the message from the user
    setConversation((prev) => [...prev, ...userMessageArray]);
    setInputText('');

    // Then we send the message to the server and handle the response
    try {
      const response = await fetch('http://0.0.0.0:5000/conversation', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Access-Control-Allow-Origin': '*',
          'Access-Control-Allow-Methods': 'POST'
        },
        body: JSON.stringify([...conversation, ...userMessageArray]) // <--- Changed here!
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const serverResponse: Conversation = await response.json();
      setConversation(serverResponse);
    } catch (error) {
      console.error('Fetching failed:', error);
    }
  }

  return (
    <div className="App">
      <header className="App-header">
        <div style={{ height: '80vh', overflowY: 'scroll' }}>
          {conversation.map((message, idx) => (
            <div key={idx}>
              {message.User && <p>{message.User}</p>}
              {message.Assistant && (
                <div>
                  <p>{message.Assistant[0]}</p>
                  {message.Assistant[1].map((url, idx) => (
                    <a key={idx} href={url}>Link</a>
                  ))}
                </div>
              )}
            </div>
          ))}
          <div ref={messagesEndRef} />
        </div>

        <div style={{ position: 'fixed', bottom: 0, display: 'flex', width: '100%', padding: 16, boxSizing: 'border-box' }}>
          <input type="text" onChange={(e) => setInputText(e.target.value)} value={inputText} style={{ width: '80%', marginRight: 8 }} />
          <button onClick={submit} style={{ width: '20%' }}>Submit</button>
        </div>
      </header>
    </div>
  );
}

export default App;