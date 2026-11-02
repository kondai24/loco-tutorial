import { StrictMode, useEffect, useState, type FormEvent } from 'react';
import { createRoot } from 'react-dom/client';
import { BrowserRouter, Link, Route, Routes, useLocation, useNavigate, useParams } from 'react-router-dom';
import { api, messages, type Answer, type Fields, type Question } from './api';
import './style.css';

function Errors({ errors }: { errors: string[] }) {
  return errors.length > 0 ? <div role="alert"><ul>{errors.map((error, i) => <li key={i}>{error}</li>)}</ul></div> : null;
}
function useLoad<T>(path: string) {
  const [data, setData] = useState<T>();
  const [errors, setErrors] = useState<string[]>([]);
  const [revision, setRevision] = useState(0);
  useEffect(() => {
    let current = true;
    setData(undefined); setErrors([]);
    api<T>(path).then(value => { if (current) setData(value); }).catch(error => { if (current) setErrors(messages(error)); });
    return () => { current = false; };
  }, [path, revision]);
  return { data, errors, reload: () => setRevision(value => value + 1) };
}
function List() {
  const { data, errors, reload } = useLoad<{ questions: Question[] }>('/questions');
  const [failure, setFailure] = useState<string[]>([]);
  const [busy, setBusy] = useState(false);
  async function destroy(id: number) {
    if (busy || !window.confirm('Are you sure?')) return;
    setBusy(true); setFailure([]);
    try { await api(`/questions/${id}`, 'DELETE'); reload(); }
    catch (error) { setFailure(messages(error)); }
    finally { setBusy(false); }
  }
  return <><h1>Questions</h1><Link to="/questions/new">new</Link><Errors errors={[...errors, ...failure]} />
    {!data && errors.length === 0 ? <p role="status">Loading...</p> : null}
    {errors.length > 0 ? <button onClick={reload}>Retry</button> : null}
    {data && <table><thead><tr><th>Title</th><th>Name</th><th>Content</th><th colSpan={3}>Actions</th></tr></thead>
      <tbody>{data.questions.map(question => <tr key={question.id}><td>{question.title}</td><td>{question.name}</td><td>{question.content}</td>
        <td><Link to={`/questions/${question.id}`}>Show</Link></td><td><Link to={`/questions/${question.id}/edit`}>Edit</Link></td>
        <td><button className="link" disabled={busy} onClick={() => void destroy(question.id)}>Destroy</button></td></tr>)}</tbody></table>}
  </>;
}
function QuestionForm({ question }: { question?: Question }) {
  const [fields, setFields] = useState<Fields>(question ?? { title: '', name: '', content: '' });
  const [errors, setErrors] = useState<string[]>([]);
  const [busy, setBusy] = useState(false);
  const navigate = useNavigate();
  async function submit(event: FormEvent) {
    event.preventDefault(); if (busy) return;
    setBusy(true); setErrors([]);
    try {
      const result = await api<{ question: Question }>(question ? `/questions/${question.id}` : '/questions', question ? 'PUT' : 'POST', fields);
      navigate(`/questions/${result.question.id}`);
    } catch (error) { setErrors(messages(error)); }
    finally { setBusy(false); }
  }
  return <><h1>{question ? 'Edit' : 'New questions'}</h1><Errors errors={errors} /><form onSubmit={submit}>
    {(['title', 'name', 'content'] as const).map(field => <div key={field}><label htmlFor={field}>{field[0].toUpperCase() + field.slice(1)}</label><br />
      {field === 'content' ? <textarea id={field} value={fields[field]} onChange={event => setFields({ ...fields, [field]: event.target.value })} /> : <input id={field} value={fields[field]} onChange={event => setFields({ ...fields, [field]: event.target.value })} />}</div>)}
    <button disabled={busy}>{busy ? 'Saving...' : question ? 'Update Question' : 'Create Question'}</button>
  </form><p><Link to="/questions">top</Link></p></>;
}
function Edit() {
  const { id } = useParams();
  const { data, errors, reload } = useLoad<{ question: Question }>(`/questions/${id}`);
  return data ? <QuestionForm key={data.question.id} question={data.question} /> : <><Errors errors={errors} />{errors.length === 0 ? <p role="status">Loading...</p> : <button onClick={reload}>Retry</button>}<Link to="/questions">top</Link></>;
}
function Show() {
  const { id } = useParams();
  const { data, errors, reload } = useLoad<{ question: Question; answers: Answer[] }>(`/questions/${id}`);
  const [answer, setAnswer] = useState({ name: '', content: '' });
  const [failure, setFailure] = useState<string[]>([]);
  const [busy, setBusy] = useState(false);
  async function submit(event: FormEvent) {
    event.preventDefault(); if (busy) return;
    setBusy(true); setFailure([]);
    try { await api(`/questions/${id}/answers`, 'POST', answer); setAnswer({ name: '', content: '' }); reload(); }
    catch (error) { setFailure(messages(error)); }
    finally { setBusy(false); }
  }
  async function destroy(answerId: number) {
    if (busy || !window.confirm('Are you sure?')) return;
    setBusy(true); setFailure([]);
    try { await api(`/questions/${id}/answers/${answerId}`, 'DELETE'); reload(); }
    catch (error) { setFailure(messages(error)); }
    finally { setBusy(false); }
  }
  return <><Errors errors={[...errors, ...failure]} />{!data && errors.length === 0 ? <p role="status">Loading...</p> : null}
    {errors.length > 0 ? <button onClick={reload}>Retry</button> : null}
    {data && <><h1>Show</h1><p>Title:<br />{data.question.title}</p><p>Name:<br />{data.question.name}</p><p>Content:<br />{data.question.content}</p>
      <h2>Answers</h2>{data.answers.map(item => <section key={item.id}><p>Name: {item.name}</p><p>Content: {item.content}</p><button className="link" disabled={busy} onClick={() => void destroy(item.id)}>Destroy answer</button></section>)}
      <h2>Add an answer</h2><form onSubmit={submit}><p><label htmlFor="answer-name">Name </label><input id="answer-name" value={answer.name} onChange={event => setAnswer({ ...answer, name: event.target.value })} /></p>
        <p><label htmlFor="answer-content">Content </label><textarea id="answer-content" value={answer.content} onChange={event => setAnswer({ ...answer, content: event.target.value })} /></p>
        <button disabled={busy}>{busy ? 'Saving...' : 'Create Answer'}</button></form></>}
    <p><Link to="/questions">top</Link></p></>;
}
function App() {
  const location = useLocation();
  return <main><Routes key={location.pathname}>
    <Route path="/" element={<List />} /><Route path="/questions" element={<List />} />
    <Route path="/questions/new" element={<QuestionForm />} /><Route path="/questions/:id" element={<Show />} /><Route path="/questions/:id/edit" element={<Edit />} />
    <Route path="*" element={<><p role="alert">ページが見つかりません。</p><Link to="/questions">top</Link></>} />
  </Routes></main>;
}
createRoot(document.getElementById('root')!).render(<StrictMode><BrowserRouter><App /></BrowserRouter></StrictMode>);
