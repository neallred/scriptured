import * as React from 'react';

interface NoResultProps {
  plausibleSearch: boolean;
  searchPending: boolean;
  searchTerm: string;
}
export default function NoResult({
  plausibleSearch,
  searchPending,
  searchTerm,
}: NoResultProps) {
  return plausibleSearch && !searchPending
    ? <div>No results matched {searchTerm}. Try altering your search</div>
    : null
}


