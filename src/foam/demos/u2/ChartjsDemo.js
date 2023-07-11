/**
 * @license
 * Copyright 2023 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

//Demo1: Using CHART from library to directly Bar chart.
const ctx = document.getElementById('barChat')
const bar1 = new org.chartjs.Lib.CHART(ctx, {
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
    responsive: true
  }
})


//Demo2: Using org.chartjs.Bar2 to create Bar chart
const bar = org.chartjs.Bar2.create({
  data: [{
    label: '# of Votes',
    data: [12, 19, 3, 5, 2, 3],
    borderWidth: 1
  }],
  config: {
    options: {
      responsive: true
    }
  }
})
bar.write();