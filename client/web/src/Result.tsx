import * as React from 'react';

interface ResultProps {
  displayString: string
}

export default function Result({
  displayString
}: ResultProps) {
  return <li dangerouslySetInnerHTML={{__html: displayString}} />
}

