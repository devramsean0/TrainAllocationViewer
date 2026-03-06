async function localFetch(url: string, data: RequestInit): Promise<Response> {
    data.headers = {
        "User-Agent": "TAC/Web"
    }
    return await fetch(url, data);
};


export async function retrieveAllocationList(fleet: string, date: string, originLoc?: string | undefined, destLoc?: string | undefined): Promise<Allocation[]> {
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
                        resourceGroup {
                            id,
                        },
                        destLocation {
                            crs,
                            nlc,
                            uic
                        },
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

export async function retrieveAllocation(id: number): Promise<Allocation[]> {
    let body = {
            query: `
                query getAllocationList ($id: Int) {
                    allocations(id: $id) {
                        id
                        originDatetime
                        originLocation {
                            id
                            nlc
                            stanox
                            tiploc
                            crs
                            uic
                            nlcdesc
                            axis
                            nlcdesc16
                        }
                        originCountryCodeIso
                        originSubsidiaryInformationCode
                        originSubsidiaryInformationCompany
                        date
                        destLocation {
                            id
                            nlc
                            stanox
                            tiploc
                            crs
                            uic
                            nlcdesc
                            axis
                            nlcdesc16
                        }
                        destCountryCodeIso
                        destSubsidiaryInformationCode
                        destSubsidiaryInformationCompany
                        destDatetime
                        allocationOriginDatetime
                        allocationOriginLocation {
                            id
                            nlc
                            stanox
                            tiploc
                            crs
                            uic
                            nlcdesc
                            axis
                            nlcdesc16
                        }
                        allocationOriginCountryCodeIso
                        allocationOriginSubsidiaryInformationCode
                        allocationOriginSubsidiaryInformationCompany
                        allocationDestDatetime
                        allocationDestLocation {
                            id
                            nlc
                            stanox
                            tiploc
                            crs
                            uic
                            nlcdesc
                            axis
                            nlcdesc16
                        }
                        allocationDestCountryCodeIso
                        allocationDestSubsidiaryInformationCode
                        allocationDestSubsidiaryInformationCompany
                        sequenceNumber
                        resourceGroupPosition
                        diagramNo
                        originMiles
                        destinationMiles
                        reversed
                        resourceGroup {
                          id
                          fleet
                          resourceType
                          status
                          endOfDayMiles
                        },
                        vehicles {
                          id,
                          livery,
                          decor,
                          vehicleType,
                          specificType,
                          resourcePosition,
                          lengthValue,
                          lengthMeasure,
                          weight,
                          specialCharacteristics,
                          seatCount,
                          cabCount,
                          dateEnteredService,
                          dateRegistered,
                          category,
                          brakeType,
                          maxSpeed
                        }
                    }
                }
            `,
            variables: {
                id
            }
        };
    let res = await localFetch("http://127.0.0.1:3001/", {
        method: "POST",
        body: JSON.stringify(body)
    })
    return (await res.json() as AllocationListResRaw).data.allocations;
}


interface AllocationListResRaw {
    data: {
        allocations: Allocation[]
    }
}

export type Maybe<T> = T | null
export type Exact<T extends { [key: string]: unknown }> = {
  [K in keyof T]: T[K]
}
export type MakeOptional<T, K extends keyof T> = Omit<T, K> &
  { [SubKey in K]?: Maybe<T[SubKey]> }
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> &
  { [SubKey in K]: Maybe<T[SubKey]> }
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string
  String: string
  Boolean: boolean
  Int: number
  Float: number
}

export type Allocation = {
  __typename?: "Allocation"
  id?: Maybe<Scalars["Int"]>
  originDatetime: Scalars["String"]
  originCountryCodeIso?: Maybe<Scalars["String"]>
  originSubsidiaryInformationCode?: Maybe<Scalars["String"]>
  originSubsidiaryInformationCompany?: Maybe<Scalars["String"]>
  date?: Maybe<Scalars["String"]>
  destCountryCodeIso?: Maybe<Scalars["String"]>
  destSubsidiaryInformationCode?: Maybe<Scalars["String"]>
  destSubsidiaryInformationCompany?: Maybe<Scalars["String"]>
  destDatetime: Scalars["String"]
  allocationOriginDatetime: Scalars["String"]
  allocationOriginCountryCodeIso?: Maybe<Scalars["String"]>
  allocationOriginSubsidiaryInformationCode?: Maybe<Scalars["String"]>
  allocationOriginSubsidiaryInformationCompany?: Maybe<Scalars["String"]>
  allocationDestDatetime: Scalars["String"]
  allocationDestCountryCodeIso?: Maybe<Scalars["String"]>
  allocationDestSubsidiaryInformationCode?: Maybe<Scalars["String"]>
  allocationDestSubsidiaryInformationCompany?: Maybe<Scalars["String"]>
  sequenceNumber?: Maybe<Scalars["Int"]>
  resourceGroupPosition?: Maybe<Scalars["Int"]>
  diagramNo?: Maybe<Scalars["String"]>
  originMiles?: Maybe<Scalars["Int"]>
  destinationMiles?: Maybe<Scalars["Int"]>
  reversed?: Maybe<Scalars["String"]>
  originLocation?: Maybe<Location>
  destLocation?: Maybe<Location>
  allocationOriginLocation?: Maybe<Location>
  allocationDestLocation?: Maybe<Location>
  resourceGroup?: Maybe<ResourceGroup>
  vehicles: Array<Vehicle>
}

export type Location = {
  __typename?: "Location"
  id?: Maybe<Scalars["Int"]>
  nlc: Scalars["String"]
  stanox?: Maybe<Scalars["String"]>
  tiploc?: Maybe<Scalars["String"]>
  crs?: Maybe<Scalars["String"]>
  uic?: Maybe<Scalars["String"]>
  nlcdesc?: Maybe<Scalars["String"]>
  axis?: Maybe<Scalars["String"]>
  nlcdesc16?: Maybe<Scalars["String"]>
}

export type Query = {
  __typename?: "Query"
  hello: Scalars["String"]
  locations?: Maybe<Array<Location>>
  resourceGroups?: Maybe<Array<ResourceGroup>>
  vehicles?: Maybe<Array<Vehicle>>
  allocations?: Maybe<Array<Allocation>>
}

export type QueryLocationsArgs = {
  nlc?: Maybe<Scalars["String"]>
}

export type QueryResourceGroupsArgs = {
  id?: Maybe<Scalars["String"]>
  specificType?: Maybe<Scalars["String"]>
  uniqueFleet?: Maybe<Scalars["Boolean"]>
}

export type QueryVehiclesArgs = {
  id?: Maybe<Scalars["Int"]>
  livery?: Maybe<Scalars["String"]>
  decor?: Maybe<Scalars["String"]>
  vehicleType?: Maybe<Scalars["String"]>
  specificType?: Maybe<Scalars["String"]>
  resourceGroupId?: Maybe<Scalars["String"]>
}

export type QueryAllocationsArgs = {
  id?: Maybe<Scalars["Int"]>
  date?: Maybe<Scalars["String"]>
  originLocation?: Maybe<Scalars["String"]>
  destLocation?: Maybe<Scalars["String"]>
  allocationOriginLocation?: Maybe<Scalars["String"]>
  allocationDestLocation?: Maybe<Scalars["String"]>
  resourceGroupId?: Maybe<Scalars["String"]>
  fleet?: Maybe<Scalars["String"]>
}

export type ResourceGroup = {
  __typename?: "ResourceGroup"
  id: Scalars["String"]
  fleet: Scalars["String"]
  resourceType?: Maybe<Scalars["String"]>
  status?: Maybe<Scalars["String"]>
  endOfDayMiles?: Maybe<Scalars["String"]>
}

export type Vehicle = {
  __typename?: "Vehicle"
  id?: Maybe<Scalars["Int"]>
  livery: Scalars["String"]
  decor?: Maybe<Scalars["String"]>
  vehicleType: Scalars["String"]
  specificType: Scalars["String"]
  resourcePosition?: Maybe<Scalars["Int"]>
  plannedResourceGroup?: Maybe<Scalars["String"]>
  lengthValue?: Maybe<Scalars["String"]>
  lengthMeasure?: Maybe<Scalars["String"]>
  weight?: Maybe<Scalars["Int"]>
  seatCount?: Maybe<Scalars["Int"]>
  cabCount?: Maybe<Scalars["Int"]>
  dateEnteredService?: Maybe<Scalars["String"]>
  dateRegistered?: Maybe<Scalars["String"]>
  category?: Maybe<Scalars["String"]>
  brakeType?: Maybe<Scalars["String"]>
  maxSpeed?: Maybe<Scalars["String"]>
  resourceGroup?: Maybe<ResourceGroup>
  specialCharacteristics?: Maybe<Scalars["String"]>
}
