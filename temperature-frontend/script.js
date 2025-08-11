document.getElementById('conversionForm').addEventListener('submit', async (e) => {
  e.preventDefault();
  const value = parseFloat(document.getElementById('value').value);
  const fromUnit = document.getElementById('fromUnit').value;
  const toUnit = document.getElementById('toUnit').value;
  
  if (isNaN(value)) {
      document.getElementById('result').innerText = 'Invalid value';
      return;
  }
  
  try {
      const response = await fetch('/api/convert', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ value, from_unit: fromUnit, to_unit: toUnit })
      });
      if (!response.ok) throw new Error('Conversion failed');
      const data = await response.json();
      document.getElementById('result').innerText = `Result: ${data.result}`;
  } catch (error) {
      document.getElementById('result').innerText = error.message;
  }
});