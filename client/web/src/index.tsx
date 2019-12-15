import * as ReactDOM from 'react-dom';
import * as React from 'react';

import * as wasm from "wasm-verily-client";

import NoResult from './NoResult';
import Result from './Result';

interface FormProps {
}

function debounce(func: Function, wait = 1000) {
  let timeout: number = 0;
  function debounced(...args: any[]) {
    clearTimeout(timeout);
    timeout = window.setTimeout(() => {
      console.log('applying func with args:', args);
      func.apply(this, args);
    }, wait);
  };
  debounced.cancel = () => clearTimeout(timeout);
  return debounced
}

const noResults: string[] = [];
function Form({}: FormProps) {
  const [searchTerm, setSearchTerm] = React.useState("");
  const [results, setResults] = React.useState<string[]>(noResults);

  const debouncedFullTextSearch = React.useCallback(debounce((currentSearchTerm: string) => {
    const newResults = wasm.full_match_search(currentSearchTerm);
    setResults(newResults);
  }), []);

  React.useEffect(() => {
    searchTerm
      ? debouncedFullTextSearch(searchTerm)
      : debouncedFullTextSearch.cancel();
  }, [searchTerm]);
  setSearchTerm
  const boundSetSearchTerm = React.useCallback(e => setSearchTerm(e.target.value), []);

  return <form>
    <input placeholder="Enter a search" value={searchTerm} onChange={boundSetSearchTerm} />
    {results.length
      ? results.map(x => <Result key={x} displayString={x} />)
      : <NoResult searchTerm={searchTerm} />
    }
    <div>
      {}
    </div>
  </form>

}

ReactDOM.render(<Form />, document.getElementById('verily-root'));
