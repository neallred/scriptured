import * as React from 'react';

interface NoResultProps {
  searchTerm: string
}
export default function NoResult({
  searchTerm
}: NoResultProps) {
  return searchTerm
    ? <div>No results matched {searchTerm}. Try altering your search</div>
    : <div>Enter a search</div>
}


