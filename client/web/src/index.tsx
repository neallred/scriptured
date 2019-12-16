import * as ReactDOM from 'react-dom';
import * as React from 'react';

import * as wasm from "wasm-verily-client";

import NoResult from './NoResult';
import Result from './Result';
import Preferences from './Preferences';
import { debounce } from './utils';

interface FormProps {
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

  return <div>
    <form>
      <input placeholder="Enter a search" value={searchTerm} onChange={boundSetSearchTerm} />
      {results.length
        ? results.map(x => <Result key={x} displayString={x} />)
        : <NoResult searchTerm={searchTerm} />
      }
      <div>
        {}
      </div>
    </form>
    <Preferences />
  </div>

}

ReactDOM.render(<Form />, document.getElementById('verily-root'));
