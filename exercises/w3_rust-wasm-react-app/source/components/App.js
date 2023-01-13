import React, { useEffect, useState } from 'react';
import * as Hoho from 'rust-wasm-app-template';

let baz = new Hoho.Baz(10);

const App = () => {
    const [sum, setNum] = useState(Hoho.add_unumbers(40, 1));
    useEffect(() => {
        setNum(sum + 1);
        baz.field2 = 100

    }, [])
    return (<div className="App">
        <h1>Hello World!</h1>
        <div>The sum is: {sum}</div>
        <div>baz field1: {baz.field1}</div>
        <div>baz field2: {baz.field2}</div>
    </div>
    );
};

export default App;

