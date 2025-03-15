import { useState, useRef, useEffect } from 'react';
import './App.css';
import katex from 'katex';
import 'katex/dist/katex.min.css';

interface MathComponentProps {
    texExpression: string;
    displayMode?: boolean;
}

function MathComponent({ texExpression, ...props }: MathComponentProps) {
    const containerRef = useRef<HTMLSpanElement | null>(null);

    useEffect(() => {
        if (containerRef.current) {
            katex.render(texExpression, containerRef.current, {
                throwOnError: false, // Handle errors gracefully
                displayMode: props.displayMode || false,
            });
        }
    }, [texExpression, props.displayMode]);

    return <span ref={containerRef} />;
}

function App() {
    const [expression, setExpression] = useState('');
    const [latex, setLatex] = useState('');
    const [result, setResult] = useState('');
    const [error, setError] = useState('');

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setError(''); // Clear any previous errors
        setLatex(''); // Clear previous result
        setResult(''); // Clear previous result

        try {
            const response = await fetch(`http://localhost:3000/simplify/${encodeURIComponent(expression)}`);
            if (!response.ok) {
                throw new Error('Failed to fetch the LaTeX expression.');
            }
            const data = await response.json(); // Assuming the API returns plain text
            setLatex(data.simplified);
            setResult(data.result);
        } catch (err) {
            setError('Error simplifying the expression. Please try again.');
        }
    };

    return (
        <div style={{ padding: '20px' }}>
            <h1>Equation Analyzer</h1>
            <form onSubmit={handleSubmit} style={{ marginBottom: '20px' }}>
                <input
                    type="text"
                    value={expression}
                    onChange={(e) => setExpression(e.target.value)}
                    placeholder="Enter a mathematical expression"
                    style={{ width: '300px', padding: '10px' }}
                />
                <button type="submit" style={{ marginLeft: '10px', padding: '10px' }}>
                    Simplify
                </button>
            </form>
            {error && <p style={{ color: 'red' }}>{error}</p>}
            {latex && (
                <div>
                    <h2>Result:</h2>
                    <MathComponent texExpression={latex} displayMode={true} />
                </div>
            )}
            {result && (
                <div>
                    <h2>Result:</h2>
                    <p>{result}</p>
                </div>
            )}
        </div>
    );
}

export default App;
