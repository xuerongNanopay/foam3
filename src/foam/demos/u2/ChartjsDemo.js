/**
 * @license
 * Copyright 2023 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

//**NOTE: below demo is based on Chart.js v4.3.0.

//Demo1: Using CHART from library to directly Bar chart.
const ctx = document.querySelector('canvas')
const bar1 = new org.chartjs.Lib.CHART(ctx.getContext('2d'), {
  type: 'bar',
  data: {
    labels: ['Jul 7', 'Jul 8', 'Jul 9', 'Jul 10', 'Jul 11', 'Jul 12'],
    datasets: [
      {
        label: 'under 3K',
        data: [12, 19, 3, 5, 2, 3],
        borderWidth: 1
      },
      {
        label: 'over 3K',
        data: [20, 1, 1, 5, 22, 13],
        borderWidth: 1
      },
      {
        label: 'total',
        data: [32, 20, 4, 10, 24, 16],
        borderWidth: 1
      }
    ]
  },
  options: {
    responsive: false
  }
})

//Demo2: Using org.chartjs.Bar2 to create Bar chart
const bar = org.chartjs.Bar2.create({
  width: 400,
  height: 200,
  data: {
    labels: ['Jul 7', 'Jul 8', 'Jul 9', 'Jul 10', 'Jul 11', 'Jul 12'],
    datasets: [
      {
        label: 'under 3K',
        data: [12, 19, 3, 5, 2, 3],
        borderWidth: 1
      },
      {
        label: 'over 3K',
        data: [20, 1, 1, 5, 22, 13],
        borderWidth: 1
      },
      {
        label: 'total',
        data: [32, 20, 4, 10, 24, 16],
        borderWidth: 1
      }
    ]
  },
  options: {
    responsive: false // default is false
  }
})
bar.write();

//Demo3: Using org.chartjs.Line2 to create Line chart.
const line = org.chartjs.Line2.create({
  width: 400,
  height: 200,
  data: {
    labels: ['Jul 7', 'Jul 8', 'Jul 9', 'Jul 10', 'Jul 11', 'Jul 12', 'Jul 13', 'Jul 14', 'Jul 15', 'Jul 16'],
    datasets: [
      {
        label: 'under 3K',
        data: [12, 19, 3, 5, 2, 3, 3, 5, 2, 3],
        borderWidth: 1
      },
      {
        label: 'over 3K',
        data: [20, 1, 1, 5, 22, 13, 3, 5, 2, 3],
        borderWidth: 1
      },
      {
        label: 'total',
        data: [32, 20, 4, 10, 24, 16, 3, 5, 2, 3],
        borderWidth: 1
      }
    ]
  },
  options: {
    responsive: false,
    plugins: {
      tooltip: {
        callbacks: {
          label: function(context) {
            return context.dataset.label + ": " + context.formattedValue + '%'
          }
        }
      }
    },
    scales: {
      x: {
        title: {
          display: true,
          text: '10 Days Conversion'
        }
      },
      y: {
        ticks: {
          beginAtZero: 0,
          min: 0,
          max: 100,
          callback: function(value, _, _) {
            return `${value}.0%`
          }
        },
        scaleLabel: {
          display: true,
          labelString: "Percentage"
        }
      }
    }
  }
})

line.write();