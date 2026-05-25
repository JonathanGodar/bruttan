// src/api/mutator/custom-fetch.ts
export const customFetch = async <T>(
  url: string,
  options: RequestInit,
): Promise<T> => {
  const response = await fetch(`http://localhost:3000/api${url}`, options);

  console.log("what the fak hatar dethär");


  if (!response.ok) {
    throw new Error(response.statusText);
  }

  return response.json();
};
