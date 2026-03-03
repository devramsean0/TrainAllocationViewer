async function localFetch(url: string, data: RequestInit): Promise<Response> {
    data.headers = {
        "User-Agent": "TAC/Web"
    }
    return await fetch(url, data);
};


export async function retrieveAllocationList(fleet: string, date: string, originLoc?: string | undefined, destLoc?: string | undefined): Promise<AllocationList[]> {
    let body = {
            query: `
                query getAllocationList ($fleet: String, $date: String, $originLocation: String, $destLocation: String) {
                    allocations(fleet: $fleet, date: $date, originLocation: $originLocation, destLocation: $destLocation) {
                        id,
                        originLocation {
                            crs,
                            nlc,
                            uic
                        }
                        originDatetime,
                        resourceGroup {
                            id,
                        },
                        destLocation {
                            crs,
                            nlc,
                            uic
                        },
                        destDatetime
                    }
                }
            `,
            variables: {
                fleet,
                date,
                originLocation: originLoc,
                destLocation: destLoc
            }
        };
    //console.log(fleet, body)
    let res = await localFetch("http://127.0.0.1:3001/", {
        method: "POST",
        body: JSON.stringify(body)
    })
    //console.log(res)
    //console.log(await res.text())
    return (await res.json() as AllocationListResRaw).data.allocations;
    //return []
}

interface AllocationListResRaw {
    data: {
        allocations: AllocationList[]
    }
}

interface AllocationList {
    id: number,
    date: string,
    originDatetime: string,
    destDatetime: string,
    destLocation: {
        crs: string | undefined,
        uic: string | undefined,
        nlc: string | undefined
    },
    resourceGroup: {
        id: String
    },
    originLocation: {
        crs: string | undefined,
        uic: string | undefined,
        nlc: string | undefined
    }
}