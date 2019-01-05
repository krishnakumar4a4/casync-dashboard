var index_id;

console.log("highcharts", Highcharts);

$(document).ready(function() {
    console.log("on page load");
    $("#graph-container").css("display","none");
});

$(document).on("click", "#graph-container-close", function() {
    $("#graph-container").css("display","none");
});

// How to call javascript library from rust?
$(document).on("click", "#btn-view-chunks-graph", function() {
    $("#graph-container").css("display","inline-block");
    $.ajax({
        url: "http://localhost:8001/chunks",
        crossDomain: true,
        cache: false,
        contentType: "text/plain",
        processData: false,
        type: 'get',
        success: function(data) {
            var chunk_ids = [];
            var download_counts = [];
            for(i=0; i<data.length;i++) {
                chunk_ids = chunk_ids.concat(data[i].id);
                download_counts = download_counts.concat(data[i].stats_download_count)
            }
            Highcharts.chart('graph-container', {
                chart: {
                    type: 'area',
                    spacingBottom: 30
                },
                title: {
                    text: 'Chunk download trends'
                },
                subtitle: {
                    text: '* Lists all the available chunks',
                    floating: true,
                    align: 'right',
                    verticalAlign: 'bottom',
                    y: 15
                },
                legend: {
                    layout: 'vertical',
                    align: 'left',
                    verticalAlign: 'top',
                    x: 100,
                    y: 70,
                    floating: true,
                    borderWidth: 1,
                    backgroundColor: (Highcharts.theme && Highcharts.theme.legendBackgroundColor) || '#FFFFFF'
                },
                xAxis: {
                    categories: chunk_ids
                },
                yAxis: {
                    title: {
                        text: 'Y-Axis'
                    },
                    labels: {
                        formatter: function () {
                            return this.value;
                        }
                    }
                },
                tooltip: {
                    formatter: function () {
                        return '<b>' + this.series.name + '</b><br/>' +
                            this.x + ': ' + this.y;
                    }
                },
                plotOptions: {
                    area: {
                        fillOpacity: 0.5
                    }
                },
                credits: {
                    enabled: false
                },
                series: [{
                    name: 'Chunks All',
                    data: download_counts
                }]
            });
        },
        fail: function() {
            $("#graph-container").css("display","none");
        }
    });
});

$(document).on("click", "#btn-view-indexes-graph", function() {
    $("#graph-container").css("display","inline-block");
    $.ajax({
        url: "http://localhost:8001/indexes",
        crossDomain: true,
        cache: false,
        contentType: "text/plain",
        processData: false,
        type: 'get',
        success: function(data) {
            var index_ids = [];
            var download_counts = [];
            for(i=0; i<data.length;i++) {
                index_ids = index_ids.concat(data[i].id);
                download_counts = download_counts.concat(data[i].stats_confirmed_download_count)
            }
            Highcharts.chart('graph-container', {
                chart: {
                    type: 'area',
                    spacingBottom: 30
                },
                title: {
                    text: 'Index download trends'
                },
                subtitle: {
                    text: '* Lists all the available indexes',
                    floating: true,
                    align: 'right',
                    verticalAlign: 'bottom',
                    y: 15
                },
                legend: {
                    layout: 'vertical',
                    align: 'left',
                    verticalAlign: 'top',
                    x: 100,
                    y: 70,
                    floating: true,
                    borderWidth: 1,
                    backgroundColor: (Highcharts.theme && Highcharts.theme.legendBackgroundColor) || '#FFFFFF'
                },
                xAxis: {
                    categories: index_ids
                },
                yAxis: {
                    title: {
                        text: 'Y-Axis'
                    },
                    labels: {
                        formatter: function () {
                            return this.value;
                        }
                    }
                },
                tooltip: {
                    formatter: function () {
                        return '<b>' + this.series.name + '</b><br/>' +
                            this.x + ': ' + this.y;
                    }
                },
                plotOptions: {
                    area: {
                        fillOpacity: 0.5
                    }
                },
                credits: {
                    enabled: false
                },
                series: [{
                    name: 'Indexes All',
                    data: download_counts
                }]
            });
        },
        fail: function() {
            $("#graph-container").css("display","none");
        }
    });
});


// Could not find alternative to getElementById in rust to read the file upload content from an element 
$(document).on("click", "#btn-blob", function() {
    var index_file_name = $("#input-blob-index-name").val();
    if(index_file_name.length > 0) {
        var file = $("#input-blob").prop("files")[0];   // Getting the properties of file from file field
        if(file) {
            $("#progress-blob").css("display","inline-block");
            var file_data;
            const reader = new FileReader();
            reader.readAsArrayBuffer(file);
            console.log('LOADING', reader.readyState); // readyState will be 1
            reader.onloadend = function () {
                console.log('DONE', reader.readyState); // readyState will be 2
                file_data = reader.result;
                file_name_to_be_uploaded = reader.fileName;
                console.log("file_data loadend: ",file_data);
                    $.ajax({
                        url: "http://localhost:8001/upload/blob?blob_name="+file_name_to_be_uploaded+"&index_name=" + index_file_name + ".caibx",
                        crossDomain: true,
                        cache: false,
                        contentType: "text/plain",
                        processData: false,
                        data: file_data,                         // Setting the data attribute of ajax with file_data
                        type: 'post',
                        success: function() {
                            alert("Blob file uploaded successfully");
                            $("#progress-blob").css("display","none");
                            $("#input-blob").val('');
                            $("#input-blob-index-name").val('');
                        },
                        fail: function() {
                            alert("Blob file failed to upload, retry");
                            $("#progress-index").css("display","none");
                        }
                })
            };
        } else {
            alert("Select blob to upload");
        }
    } else {
        alert("Index file name cannot be empty");
    }
})

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
            file_name_to_be_uploaded = file.name;
                $.ajax({
                    url: "http://localhost:8001/upload/index?name="+file_name_to_be_uploaded+"&version=\"6.9\"",
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
                        $("#input-index").val('');
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
    } else {
        alert("Select index file to upload");
    }
})

//TODO: Add new endpoint in backend for uploading chunks
$(document).on("click", "#btn-chunks", function() {
    $("#progress-chunks").css("display","inline-block");
    var files = $("#input-chunks").prop("files") 
    if (files.length > 0) {
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
                            $("#input-chunks").val('');
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
    } else {
        alert("Select atleast one or more chunk files to upload");
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