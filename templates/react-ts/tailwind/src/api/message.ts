export const fetchMessage = async () => {
  const res = await fetch("http://localhost:5000/api/message");
  if (!res.ok) throw new Error(`HTTP ${res.status}`);
  return res.json();
};
