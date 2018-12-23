// TODO: Add new endpoint in backend for uploading blob
// $(document).on("click", "#btn-blob", function() {
// 	var file = $("#input-blob").prop("files")[0];   // Getting the properties of file from file field
//     var file_data;
//     const reader = new FileReader();
//     reader.readAsArrayBuffer(file);
//     console.log('LOADING', reader.readyState); // readyState will be 1
//     reader.onloadend = function () {
//         console.log('DONE', reader.readyState); // readyState will be 2
//         file_data = reader.result;
//         console.log("file_data loadend: ",file_data);
//             $.ajax({
//                 url: "http://localhost:8001/upload/index?name=test1.caidx&version=\"6.9\"",
//                 crossDomain: true,
//                 cache: false,
//                 contentType: "text/plain",
//                 processData: false,
//                 data: file_data,                         // Setting the data attribute of ajax with file_data
//                 type: 'post'
//         })
//     };
// })

$(document).on("click", "#btn-index", function() {
	var file = $("#input-index").prop("files")[0];   // Getting the properties of file from file field
    var file_data;
    const reader = new FileReader();
    reader.readAsArrayBuffer(file);
    console.log('LOADING', reader.readyState); // readyState will be 1
    reader.onloadend = function () {
        console.log('DONE', reader.readyState); // readyState will be 2
        file_data = reader.result;
        console.log("file_data loadend: ",file_data);
            $.ajax({
                url: "http://localhost:8001/upload/index?name=test1.caidx&version=\"6.9\"",
                crossDomain: true,
                cache: false,
                contentType: "text/plain",
                processData: false,
                data: file_data,                         // Setting the data attribute of ajax with file_data
                type: 'post'
        })
    };
})

// TODO: Add new endpoint in backend for uploading chunks
// $(document).on("click", "#btn-chunks", function() {
// 	var file = $("#input-chunks").prop("files")[0];   // Getting the properties of file from file field
//     var file_data;
//     const reader = new FileReader();
//     reader.readAsArrayBuffer(file);
//     console.log('LOADING', reader.readyState); // readyState will be 1
//     reader.onloadend = function () {
//         console.log('DONE', reader.readyState); // readyState will be 2
//         file_data = reader.result;
//         console.log("file_data loadend: ",file_data);
//             $.ajax({
//                 url: "http://localhost:8001/upload/index?name=test1.caidx&version=\"6.9\"",
//                 crossDomain: true,
//                 cache: false,
//                 contentType: "text/plain",
//                 processData: false,
//                 data: file_data,                         // Setting the data attribute of ajax with file_data
//                 type: 'post'
//         })
//     };
// })