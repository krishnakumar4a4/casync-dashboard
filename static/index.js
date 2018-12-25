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
var index_id;

$(document).ready(function() {
    console.log("on page load");
});
$(document).on("click", "#btn-index", function() {
    var file = $("#input-index").prop("files")[0];   // Getting the properties of file from file field
    if(file) {
        $("#progress-index").css("display","inline-block");
        $("#btn-chunks").prop("disabled", true);
        var file_data;
        const reader = new FileReader();
        reader.readAsArrayBuffer(file);
        reader.onloadend = function () {
            file_data = reader.result;
                $.ajax({
                    url: "http://localhost:8001/upload/index?name=test1.caidx&version=\"6.9\"",
                    crossDomain: true,
                    cache: false,
                    contentType: "text/plain",
                    processData: false,
                    data: file_data,                         // Setting the data attribute of ajax with file_data
                    type: 'post',
                    success: function(res) {
                        if(res.length>0) {
                            index_id = res[0];
                        }
                        $("#btn-chunks").prop("disabled", false);
                        $("#input-chunks").prop("disabled", false);
                        alert("Index file uploaded successfully");
                        $("#progress-index").css("display","none");
                    },
                    fail: function() {
                        $("#btn-chunks").prop("disabled", true);
                        $("#input-chunks").prop("disabled", true);
                        alert("Index file failed to upload");
                        $("#progress-index").css("display","none");
                        index_id = null;
                    }
            })
        };
    }
})

//TODO: Add new endpoint in backend for uploading chunks
$(document).on("click", "#btn-chunks", function() {
    $("#progress-chunks").css("display","inline-block");
    var files = $("#input-chunks").prop("files") 
    var file_count = files.length;
    $("#progress-chunks").prop("max", file_count)
    var file_upload_success_count = 0;
    $("#progress-chunks").prop("value", file_upload_success_count)
    var file_upload_response_count = 0;
    for(var i=0; i<file_count; i++) {
        var file = files[i];
        var file_name = file.name;
        var file_data;
        const reader = new FileReader();
        reader.readAsArrayBuffer(file);
        reader.onloadend = function (e) {
            file_data = reader.result;
            file_name_to_be_uploaded = reader.fileName;
                $.ajax({
                    url: "http://localhost:8001/upload/chunk?name="+file_name_to_be_uploaded+"&index_id="+index_id,
                    crossDomain: true,
                    cache: false,
                    contentType: "text/plain",
                    processData: false,
                    data: file_data,                         // Setting the data attribute of ajax with file_data
                    type: 'post',
                    success: function() {
                        $("#btn-chunks").prop("disabled", true);
                        $("#input-chunks").prop("disabled", true);
                        file_upload_success_count = file_upload_success_count + 1;
                        file_upload_response_count = file_upload_response_count + 1;
                        $("#progress-chunks").prop("value", file_upload_success_count);
                        checkChunksUploadError(file_count, file_upload_success_count, file_upload_response_count);
                    },
                    fail: function() {
                        $("#btn-chunks").prop("disabled", false);
                        $("#input-chunks").prop("disabled", false);
                        file_upload_response_count = file_upload_response_count + 1;
                        checkChunksUploadError(file_count, file_upload_success_count, file_upload_response_count);
                    }
            })
        };
        reader.fileName = file_name;
    }
})

function checkChunksUploadError(file_count, file_upload_success_count, file_upload_response_count) {
    if (file_count == file_upload_response_count) {
        if (file_upload_success_count != file_count) {
            alert("Some files are not properly uploaded, Retry");
        } else {
            alert("Chunks uploaded successfully");
            index_id = null;
            $("#progress-chunks").css("display","none");
        }
    }
}